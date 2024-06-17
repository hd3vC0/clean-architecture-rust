use aws_config::{meta::region::RegionProviderChain, BehaviorVersion, SdkConfig};


pub async fn aws_config () -> SdkConfig {    
        let region = RegionProviderChain::default_provider()
        .or_else("us-east-1");
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(region)
            .load()
            .await;
        config
}