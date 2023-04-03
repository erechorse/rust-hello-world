use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Generate new keys
    let my_keys: Keys = Keys::generate();

    let bech32_pubkey: String = my_keys.public_key().to_bech32()?;
    println!("Bech32 PubKey: {}", bech32_pubkey);

    // Create new client
    let client = Client::new(&my_keys);

    // Add relays
    client.add_relay("wss://relay.damus.io", None).await?;

    // Connect to relays
    client.connect().await;

    let metadata = Metadata::new()
        .name("TEST")
        .display_name("test")
        .about("This is a test");

    // Update metadata
    client.set_metadata(metadata).await?;

    // Publish a text note
    client.publish_text_note("My first text note from Nostr SDK!", &[]).await?;

    // Send custom event
    let event_id = EventId::from_bech32("note1z3lwphdc7gdf6n0y4vaaa0x7ck778kg638lk0nqv2yd343qda78sf69t6r")?;
    let public_key = XOnlyPublicKey::from_bech32("npub14rnkcwkw0q5lnmjye7ffxvy7yxscyjl3u4mrr5qxsks76zctmz3qvuftjz")?;
    let event: Event = EventBuilder::new_reaction(event_id, public_key, "🧡").to_event(&my_keys)?;

    // Send custom event to all relays
    // client.send_event(event).await?;

    // Send custom event to a specific previously added relay
    client.send_event_to("wss://relay.damus.io", event).await?;

    let subscription = Filter::new()
    .limit(0)
    .kinds(vec![Kind::TextNote])
    .pubkey(my_keys.public_key());
    client.subscribe(vec![subscription]).await;

    loop {
        let mut notifications = client.notifications();
        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotification::Event(_url, event) = notification {
                match event.kind {
                    Kind::TextNote => {
                        println!("{}", event.content);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
    }

    Ok(())
}