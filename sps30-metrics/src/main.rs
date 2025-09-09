use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;
use std::env;
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::api_metrics::SubmitMetricsOptionalParams;
use datadog_api_client::datadogV2::model::MetricIntakeType;
use datadog_api_client::datadogV2::model::MetricPayload;
use datadog_api_client::datadogV2::model::MetricPoint;
use datadog_api_client::datadogV2::model::MetricResource;
use datadog_api_client::datadogV2::model::MetricSeries;
use sps30_i2c::types::AirInfo;
mod mock_sensor;

// Only include real sensor on Linux platforms
#[cfg(target_os = "linux")]
mod real_sensor;

trait Sensor {
    fn wake_up(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn start_measurement(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn read_data_ready_flag(&mut self) -> Result<bool, Box<dyn std::error::Error>>;
    fn read_measured_values(&mut self) -> Result<AirInfo, Box<dyn std::error::Error>>;
    fn stop_measurement(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn sleep(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    // Device information methods
    fn read_device_product_type(&mut self) -> Result<[u8; 32], Box<dyn std::error::Error>>;
    fn read_device_serial_number(&mut self) -> Result<[u8; 32], Box<dyn std::error::Error>>;
    fn read_firmware_version(&mut self) -> Result<[u8; 32], Box<dyn std::error::Error>>;
    fn start_fan_cleaning(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Check environment variable to determine which sensor to use
    // If ENV == "prod", use real sensor (Linux only); otherwise use mock sensor
    let env_mode = env::var("ENV").unwrap_or_else(|_| "dev".to_string());
    
    let mut sensor: Box<dyn Sensor> = if env_mode == "prod" {
        #[cfg(target_os = "linux")]
        {
            println!("Running in production mode - using real SPS30 sensor");
            Box::new(real_sensor::RealSps30::new()?)
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            println!("Production mode requested but not running on Linux!");
            println!("Falling back to mock sensor (real sensor only available on Linux)");
            Box::new(mock_sensor::MockSps30::new())
        }
    } else {
        println!("Running in development mode (ENV={})", env_mode);
        println!("Using mock sensor - set ENV=prod to use real sensor (Linux only)");
        Box::new(mock_sensor::MockSps30::new())
    };

    sensor.wake_up()?;
    sensor.start_measurement()?;

    let bytes_to_string = |bytes: [u8; 32]| -> String {
        let string_bytes: Vec<u8> = bytes.iter().take_while(|&&b| b != 0).cloned().collect();
        String::from_utf8(string_bytes).unwrap_or_else(|_| "INVALID_UTF8".to_string())
    };

    let tags = vec![
        ("product_type".to_string(), bytes_to_string(sensor.read_device_product_type()?)),
        ("serial_number".to_string(), bytes_to_string(sensor.read_device_serial_number()?)),
        ("firmware_version".to_string(), bytes_to_string(sensor.read_firmware_version()?)),
    ];

    sensor.start_fan_cleaning()?;

    println!("Starting infinite metrics collection loop...");
    println!("Press Ctrl+C to stop");

    loop {

        match sensor.read_data_ready_flag() {
            Ok(true) => {
                match sensor.read_measured_values() {
                    Ok(air_info) => {
                        println!("Air quality data: {:?}", air_info);
                        println!("Tags: {:?}", tags);
                        let body = build_metrics_payload(air_info, Some(tags.clone()));
                        
                        let configuration = datadog::Configuration::new();
                        let api = MetricsAPI::with_config(configuration);
                        let resp = api
                            .submit_metrics(body, SubmitMetricsOptionalParams::default())
                            .await;
                        
                        match resp {
                            Ok(value) => {
                                println!("Metrics response: {:#?}", value);
                            }
                            Err(e) => {
                                eprintln!("Error submitting metrics: {:#?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading sensor values: {:?}", e);
                    }
                }
            }
            Ok(false) => {
                println!("No new data available yet");
            }
            Err(e) => {
                eprintln!("Error checking data ready flag: {:?}", e);
            }
        }

        // Sleep for 1 second before next reading
        thread::sleep(Duration::from_secs(1));
    }
}

// Helper function to create a metric series with tags
fn create_metric_series(
    metric_name: &str,
    value: f64,
    timestamp: i64,
    tags: &Option<Vec<(String, String)>>,
) -> MetricSeries {
    let mut resources = Vec::new();

    if let Some(ref tags) = tags {
        for (key, value) in tags {
            resources.push(
                MetricResource::new()
                    .name(value.to_string())
                    .type_(key.to_string())
            );
        }
    }

    let series = MetricSeries::new(
        metric_name.to_string(),
        vec![MetricPoint::new().timestamp(timestamp).value(value)],
    )
    .resources(resources)
    .type_(MetricIntakeType::GAUGE);
    
    series
}

fn build_metrics_payload(air_info: AirInfo, tags: Option<Vec<(String, String)>>) -> MetricPayload {
    let now = SystemTime::now();
    let timestamp = match now.duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(_) => 0,
    };

    let mut metrics = Vec::new();
    let base_metric_name = "air_quality";

    // Create metrics for each field in AirInfo
    let field_mappings = vec![
        ("mass_pm1_0", air_info.mass_pm1_0 as f64),
        ("mass_pm2_5", air_info.mass_pm2_5 as f64),
        ("mass_pm4_0", air_info.mass_pm4_0 as f64),
        ("mass_pm10", air_info.mass_pm10 as f64),
        ("number_pm0_5", air_info.number_pm0_5 as f64),
        ("number_pm1_0", air_info.number_pm1_0 as f64),
        ("number_pm2_5", air_info.number_pm2_5 as f64),
        ("number_pm4_0", air_info.number_pm4_0 as f64),
        ("number_pm10", air_info.number_pm10 as f64),
        ("typical_particle_size", air_info.typical_size as f64),
    ];

    for (field_name, value) in field_mappings {
        let metric_name = format!("{}.{}", base_metric_name, field_name);
        let series = create_metric_series(&metric_name, value, timestamp, &tags);
        metrics.push(series);
    }

    MetricPayload::new(metrics)
}
