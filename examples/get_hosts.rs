use twitch_api2::tmi::{Host, HostsRequestId};
use twitch_api2::TmiClient;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let mut args = std::env::args().skip(1);
    let channel_id = if let Some(Ok(id)) = args.next().map(|s| s.parse::<u64>()) {
        id
    } else if let Ok(Ok(id)) = std::env::var("TWITCH_CHANNEL_ID").map(|s| s.parse::<u64>()) {
        id
    } else {
        eprintln!(
            "Please provide a channel ID as either the first argument \
			or stored in the `TWITCH_CHANNEL_ID` environment variable"
        );
        return;
    };

    let client: TmiClient<surf::Client> = TmiClient::new();

    let response = client
        .get_hosts(true, HostsRequestId::Host(channel_id))
        .await
        .expect("`HostsRequest::Host` failed");

    match response
        .hosts
        .first()
        .expect("`get_hosts` should have returned exactly one record")
    {
        Host {
            target_id: Some(target_id),
            target_display_name: Some(target_name),
            host_display_name: Some(host_name),
            ..
        } => {
            println!("{} is hosting: {:#?}", host_name, response.hosts.first());

            let response = client
                .get_hosts(true, HostsRequestId::Target(*target_id))
                .await
                .expect("`HostsRequest::Target` failed");

            println!("Also hosting {}:", target_name);

            for host in response.hosts {
                println!(
                    "  {}",
                    host.host_display_name.unwrap_or_else(|| "[unknown]".into())
                );
            }
        }
        Host {
            target_id: None,
            host_display_name: Some(name),
            ..
        } => println!("{} is not hosting anyone.", name),
        _ => println!("Couldn't find requested host."),
    }
}
