use beer_domain::{
    drinks::{Abv, CreatedDrink, DrinkSize},
    time::UnixSeconds,
};
use serde_json::json;

#[test]
fn shared_drinks_preserve_api_field_names_and_timestamps() {
    let size = DrinkSize {
        id: "pint".into(),
        name: "Pint".into(),
        volume_ml: 500,
        description: None,
    };
    assert_eq!(
        serde_json::to_value(size).unwrap(),
        json!({"id": "pint", "name": "Pint", "volumeML": 500, "description": null})
    );

    let drink = CreatedDrink {
        id: "drink".into(),
        event_id: "event".into(),
        user_id: "user".into(),
        image_id: "drink.png".into(),
        created_at: UnixSeconds::from_seconds(1_700_000_000),
        drink_type_id: None,
        drink_size_id: Some("pint".into()),
        abv: Some(5.0),
    };
    assert_eq!(
        serde_json::to_value(drink).unwrap(),
        json!({
            "id": "drink",
            "eventId": "event",
            "userId": "user",
            "imageId": "drink.png",
            "createdAt": 1_700_000_000,
            "drinkTypeId": null,
            "drinkSizeId": "pint",
            "abv": 5.0
        })
    );
}

#[test]
fn alcohol_percentage_validation_preserves_boundaries() {
    for value in [0.0, 5.0, 100.0] {
        assert!((Abv::parse(value).unwrap().value() - value).abs() < f64::EPSILON);
    }
    for value in [-0.1, 100.1, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(Abv::parse(value).is_err());
    }
}

#[test]
fn identifiers_round_trip_as_plain_strings_including_legacy_values() {
    use beer_domain::id::{DrinkId, EventId, ImageId, UserId};
    use serde::{Serialize, de::DeserializeOwned};
    use std::fmt::Debug;

    fn round_trip<T: From<String> + Serialize + DeserializeOwned + PartialEq + Debug>(value: &str) {
        let id = T::from(value.to_owned());
        let serialized = serde_json::to_value(&id).unwrap();
        assert_eq!(serialized, json!(value));
        assert_eq!(serde_json::from_value::<T>(serialized).unwrap(), id);
    }

    round_trip::<UserId>("legacy-user");
    round_trip::<EventId>("open");
    round_trip::<DrinkId>("drink");
    round_trip::<ImageId>("drink.png");
}
