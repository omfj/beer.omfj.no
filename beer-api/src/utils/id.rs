use beer_domain::id::{DrinkId, EventId, UserId};
use nanoid::nanoid;

pub(crate) fn user_id() -> UserId {
    const ALPHABET: [char; 32] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '2', '3', '4', '5', '6', '7',
    ];
    nanoid!(24, &ALPHABET).into()
}

pub(crate) fn event_id() -> EventId {
    const LETTERS: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    format!("{}{}", nanoid!(2, &LETTERS), nanoid!(5, &DIGITS)).into()
}

pub(crate) fn drink_id() -> DrinkId {
    const HEX: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    nanoid!(32, &HEX).into()
}

#[cfg(test)]
mod tests {
    use super::{drink_id, event_id, user_id};

    #[test]
    fn generated_ids_preserve_existing_formats() {
        for _ in 0..100 {
            let user = user_id();
            assert_eq!(user.as_str().len(), 24);
            assert!(
                user.as_str()
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || (b'2'..=b'7').contains(&b))
            );

            let event = event_id();
            assert_eq!(event.as_str().len(), 7);
            assert!(event.as_str()[..2].bytes().all(|b| b.is_ascii_uppercase()));
            assert!(event.as_str()[2..].bytes().all(|b| b.is_ascii_digit()));

            let drink = drink_id();
            assert_eq!(drink.as_str().len(), 32);
            assert!(
                drink
                    .as_str()
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            );
        }
    }
}
