use icalendar::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event = Event::new()
        .summary("test event")
        .append_property(
            Property::new("TEST", "FOOBAR")
                .add_parameter("IMPORTANCE", "very")
                .add_parameter("DUE", "tomorrow")
                .done(),
        )
        .uid("my.own.id")
        .done();

    let calendar = Calendar::from([event]);

    calendar.print()?;

    Ok(())
}
