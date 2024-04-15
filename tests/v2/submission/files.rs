mod response {
    mod deserialize {
        use mini_exercism::api::v2::submission::files;
        use mini_exercism::api::v2::submission::files::File;

        #[test]
        fn test_all() {
            let json = r#"{
                "files": [
                    {
                        "filename": "src/lib.rs",
                        "content": "mod detail;\n\nuse crate::detail::Hand;\n\n/// Given a list of poker hands, return a list of those hands which win.\n///\n/// Note the type signature: this function should return _the same_ reference to\n/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.\npub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {\n    let hands: Vec<_> = hands.iter().map(|&h| Hand::new(h).unwrap()).collect();\n    let best = hands.iter().max().unwrap();\n    hands.iter().filter(|&h| h == best).map(|h| h.hand_s()).collect()\n}\n",
                        "digest": "2edfab2886de7d3aadac30d6aee983e3eb965aed"
                    },
                    {
                        "filename": "src/detail.rs",
                        "content": "mod slice_utils;\r\n\r\nuse std::cmp::Ordering;\r\nuse std::str::FromStr;\r\nuse derivative::Derivative;\r\nuse strum_macros::{EnumString, FromRepr};\r\nuse thiserror::Error;\r\nuse crate::detail::slice_utils::group_by::ClGroupBy;\r\nuse crate::detail::slice_utils::SliceUtils;\r\n\r\n#[derive(Debug, Error)]\r\npub enum Error {\r\n    #[error(\"Invalid card format `{0}`\")]\r\n    InvalidCardFormat(String),\r\n    #[error(\"Invalid card count `{0}` (should be 5)\")]\r\n    InvalidCardCount(usize),\r\n}\r\n\r\ntrait OrInvalidCardFormat<T> {\r\n    fn or_invalid_card_format(self, card_format: &str) -> T;\r\n}\r\n\r\nimpl OrInvalidCardFormat<Error> for strum::ParseError {\r\n    fn or_invalid_card_format(self, card_format: &str) -> Error {\r\n        match self {\r\n            strum::ParseError::VariantNotFound => Error::InvalidCardFormat(card_format.to_string()),\r\n        }\r\n    }\r\n}\r\n\r\nimpl<T, E> OrInvalidCardFormat<Result<T, Error>> for Result<T, E>\r\n    where\r\n        E: OrInvalidCardFormat<Error>\r\n{\r\n    fn or_invalid_card_format(self, card_format: &str) -> Result<T, Error> {\r\n        self.map_err(|e| e.or_invalid_card_format(card_format))\r\n    }\r\n}\r\n\r\n#[repr(i8)]\r\n#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumString, FromRepr)]\r\nenum CardValue {\r\n    #[strum(serialize = \"2\")]\r\n    Two,\r\n    #[strum(serialize = \"3\")]\r\n    Three,\r\n    #[strum(serialize = \"4\")]\r\n    Four,\r\n    #[strum(serialize = \"5\")]\r\n    Five,\r\n    #[strum(serialize = \"6\")]\r\n    Six,\r\n    #[strum(serialize = \"7\")]\r\n    Seven,\r\n    #[strum(serialize = \"8\")]\r\n    Eight,\r\n    #[strum(serialize = \"9\")]\r\n    Nine,\r\n    #[strum(serialize = \"10\")]\r\n    Ten,\r\n    #[strum(serialize = \"J\")]\r\n    Jack,\r\n    #[strum(serialize = \"Q\")]\r\n    Queen,\r\n    #[strum(serialize = \"K\")]\r\n    King,\r\n    #[strum(serialize = \"A\")]\r\n    Ace,\r\n}\r\n\r\nimpl CardValue {\r\n    pub fn next(&self) -> Self {\r\n        match Self::from_repr((*self as i8) + 1) {\r\n            Some(cv) => cv,\r\n            None => Self::from_repr(0).unwrap(),\r\n        }\r\n    }\r\n}\r\n\r\n#[derive(Copy, Clone, PartialEq, Eq, EnumString)]\r\nenum Suit {\r\n    #[strum(serialize = \"H\")]\r\n    Hearts,\r\n    #[strum(serialize = \"S\")]\r\n    Spades,\r\n    #[strum(serialize = \"D\")]\r\n    Diamonds,\r\n    #[strum(serialize = \"C\")]\r\n    Clubs,\r\n}\r\n\r\n#[derive(Derivative)]\r\n#[derivative(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]\r\nstruct Card {\r\n    value: CardValue,\r\n    #[derivative(PartialEq = \"ignore\")]\r\n    #[derivative(PartialOrd = \"ignore\")]\r\n    #[derivative(Ord = \"ignore\")]\r\n    suit: Suit,\r\n}\r\n\r\nimpl Card {\r\n    pub fn new(card_s: &str) -> Result<Card, Error> {\r\n        let (value_s, suit_s) = card_s.split_at(card_s.len() - 1);\r\n        let value = CardValue::from_str(value_s).or_invalid_card_format(card_s)?;\r\n        let suit = Suit::from_str(suit_s).or_invalid_card_format(card_s)?;\r\n        Ok(Card { value, suit })\r\n    }\r\n}\r\n\r\n#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]\r\nenum HandType {\r\n    HighCard,\r\n    Pair,\r\n    TwoPairs,\r\n    ThreeOfAKind,\r\n    Straight,\r\n    Flush,\r\n    FullHouse,\r\n    FourOfAKind,\r\n    StraightFlush,\r\n}\r\n\r\n#[derive(Derivative)]\r\n#[derivative(PartialEq, Eq, PartialOrd, Ord)]\r\npub struct Hand<'a> {\r\n    #[derivative(PartialEq = \"ignore\")]\r\n    #[derivative(PartialOrd = \"ignore\")]\r\n    #[derivative(Ord = \"ignore\")]\r\n    hand_s: &'a str,\r\n    hand_type: HandType,\r\n    contributing_cards: Vec<Card>,\r\n    non_contributing_cards: Vec<Card>,\r\n}\r\n\r\nimpl<'a> Hand<'a> {\r\n    pub fn new(hand_s: &'a str) -> Result<Hand<'a>, Error> {\r\n        let mut cards = hand_s.split(' ')\r\n            .map(|card_s| Card::new(card_s))\r\n            .collect::<Result<Vec<_>, _>>()?;\r\n        if cards.len() != 5 {\r\n            return Err(Error::InvalidCardCount(cards.len()))\r\n        }\r\n        cards.sort_unstable();\r\n\r\n        let cards = &cards[..];\r\n        let hand_type: Option<_>;\r\n        let mut contributing_cards: Option<Vec<_>> = None;\r\n        let mut non_contributing_cards: Option<Vec<_>> = None;\r\n\r\n        if Self::is_straight(&cards) {\r\n            if Self::is_flush(&cards) {\r\n                hand_type = Some(HandType::StraightFlush);\r\n            } else {\r\n                hand_type = Some(HandType::Straight);\r\n            }\r\n\r\n            if Self::is_low_straight(&cards) {\r\n                let mut cc = Vec::with_capacity(5);\r\n                cc.push(cards[4]);\r\n                cc.append(&mut cards[0..=3].into());\r\n                contributing_cards = Some(cc);\r\n            }\r\n        } else if Self::is_flush(&cards) {\r\n            hand_type = Some(HandType::Flush);\r\n        } else if Self::is_full_house(&cards) {\r\n            hand_type = Some(HandType::FullHouse);\r\n            if cards[1] == cards[2] {\r\n                contributing_cards = Some(cards[0..=2].into());\r\n                non_contributing_cards = Some(cards[3..=4].into());\r\n            } else {\r\n                contributing_cards = Some(cards[2..=4].into());\r\n                non_contributing_cards = Some(cards[0..=1].into());\r\n            }\r\n        } else {\r\n            // Once https://doc.rust-lang.org/std/primitive.slice.html#method.group_by\r\n            // is stabilized, we could switch to that instead.\r\n            let mut chunks: Vec<_> = cards.cl_group_by(|a, b| a == b).collect();\r\n            chunks.sort_unstable_by(|&c1, &c2| {\r\n                match c1.len().cmp(&c2.len()) {\r\n                    Ordering::Equal => c1[0].cmp(&c2[0]),\r\n                    cmp => cmp,\r\n                }\r\n            });\r\n\r\n            hand_type = match chunks.last().unwrap().len() {\r\n                4 => Some(HandType::FourOfAKind),\r\n                3 => Some(HandType::ThreeOfAKind),\r\n                2 => match chunks.iter().filter(|&c| c.len() == 2).count() {\r\n                    2 => {\r\n                        contributing_cards = Some(chunks[(chunks.len() - 2)..chunks.len()].to_flattened_vec());\r\n                        non_contributing_cards = Some(chunks[0].into());\r\n                        Some(HandType::TwoPairs)\r\n                    },\r\n                    _ => Some(HandType::Pair),\r\n                },\r\n                _ => Some(HandType::HighCard),\r\n            };\r\n\r\n            contributing_cards = contributing_cards.or_else(|| Some(chunks.last().unwrap().to_vec()));\r\n            non_contributing_cards = non_contributing_cards.or_else(|| Some(chunks[0..(chunks.len() - 1)].to_flattened_vec()));\r\n        }\r\n\r\n        let hand_type = hand_type.unwrap();\r\n        let mut contributing_cards = contributing_cards.unwrap_or_else(|| cards.into());\r\n        let mut non_contributing_cards = non_contributing_cards.unwrap_or_else(|| Vec::new());\r\n        contributing_cards.reverse();\r\n        non_contributing_cards.reverse();\r\n\r\n        Ok(Hand {\r\n            hand_s,\r\n            hand_type,\r\n            contributing_cards,\r\n            non_contributing_cards,\r\n        })\r\n    }\r\n\r\n    pub fn hand_s(&self) -> &'a str {\r\n        self.hand_s\r\n    }\r\n\r\n    fn is_flush(cards: &[Card]) -> bool {\r\n        cards.iter().all(|c| c.suit == cards[0].suit)\r\n    }\r\n\r\n    fn is_straight(cards: &[Card]) -> bool {\r\n        cards.iter().skip(1).enumerate().all(|(i, c)| {\r\n            cards[i].value.next() == c.value ||\r\n                (i == 3 && cards[i].value == CardValue::Five && c.value == CardValue::Ace)\r\n        })\r\n    }\r\n\r\n    fn is_low_straight(cards: &[Card]) -> bool {\r\n        Self::is_straight(cards) && cards[3].value == CardValue::Five && cards[4].value == CardValue::Ace\r\n    }\r\n\r\n    fn is_full_house(cards: &[Card]) -> bool {\r\n        (cards[0] == cards[1] && cards[2] == cards[3] && cards[3] == cards[4] && cards[1] != cards[2]) ||\r\n            (cards[0] == cards[1] && cards[1] == cards[2] && cards[3] == cards[4] && cards[2] != cards[3])\r\n    }\r\n}\r\n",
                        "digest": "cdfc3584877237d5f157091b6e756254aec056f1"
                    },
                    {
                        "filename": "src/detail/slice_utils.rs",
                        "content": "pub mod group_by;\r\n\r\npub trait SliceUtils<T> {\r\n    fn to_flattened_vec(&self) -> Vec<T>;\r\n}\r\n\r\nimpl<T: Copy> SliceUtils<T> for [&[T]] {\r\n    fn to_flattened_vec(&self) -> Vec<T> {\r\n        self.iter().map(|&c| c.to_vec()).flatten().collect()\r\n    }\r\n}\r\n",
                        "digest": "2bdcb3f11084cd7d082ff08efdda21dfb50fca5a"
                    },
                    {
                        "filename": "src/detail/slice_utils/group_by.rs",
                        "content": "pub struct GroupBy<'a, T: 'a, F: FnMut(&T, &T) -> bool> {\r\n    s: &'a [T],\r\n    len: usize,\r\n    i: usize,\r\n    f: F,\r\n}\r\n\r\npub trait ClGroupBy<'a, T: 'a> {\r\n    fn cl_group_by<F: FnMut(&T, &T) -> bool>(&self, f: F) -> GroupBy<'a, T, F>;\r\n}\r\n\r\nimpl<'a, T: 'a> ClGroupBy<'a, T> for &'a [T] {\r\n    fn cl_group_by<F: FnMut(&T, &T) -> bool>(&self, f: F) -> GroupBy<'a, T, F> {\r\n        GroupBy::new(self, f)\r\n    }\r\n}\r\n\r\nimpl<'a, T: 'a, F: FnMut(&T, &T) -> bool> GroupBy<'a, T, F> {\r\n    fn new(s: &'a [T], f: F) -> Self {\r\n        GroupBy {\r\n            s,\r\n            len: s.len(),\r\n            i: 0,\r\n            f,\r\n        }\r\n    }\r\n}\r\n\r\nimpl<'a, T: 'a, F: FnMut(&T, &T) -> bool> Iterator for GroupBy<'a, T, F> {\r\n    type Item = &'a [T];\r\n\r\n    fn next(&mut self) -> Option<Self::Item> {\r\n        match self.i {\r\n            end if end == self.len => None,\r\n            beg => {\r\n                let cur = &self.s[beg];\r\n                self.i += 1;\r\n                while self.i < self.len && (self.f)(cur, &self.s[self.i]) {\r\n                    self.i += 1;\r\n                }\r\n                Some(&self.s[beg..self.i])\r\n            }\r\n        }\r\n    }\r\n}\r\n\r\nimpl<'a, T: 'a, F: FnMut(&T, &T) -> bool> ::std::iter::FusedIterator for GroupBy<'a, T, F> {}\r\n",
                        "digest": "b332b7e8998e9711acce79b8c3b1f65610d2cf99"
                    },
                    {
                        "filename": "Cargo.toml",
                        "content": "[package]\nedition = \"2021\"\nname = \"poker\"\nversion = \"1.1.0\"\n\n[dependencies]\nderivative = \"2.2.0\"\nstrum = \"0.24.1\"\nstrum_macros = \"0.24.3\"\nthiserror = \"1.0.40\"\n",
                        "digest": "9ad1c8abd08fcc3111eaf728a9fb1f3717d10ad8"
                    }
                ]
            }"#;

            let expected = files::Response {
                files: vec![
                    File {
                        filename: "src/lib.rs".into(),
                        content: "mod detail;\n\nuse crate::detail::Hand;\n\n/// Given a list of poker hands, return a list of those hands which win.\n///\n/// Note the type signature: this function should return _the same_ reference to\n/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.\npub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {\n    let hands: Vec<_> = hands.iter().map(|&h| Hand::new(h).unwrap()).collect();\n    let best = hands.iter().max().unwrap();\n    hands.iter().filter(|&h| h == best).map(|h| h.hand_s()).collect()\n}\n".into(),
                        digest: "2edfab2886de7d3aadac30d6aee983e3eb965aed".into(),
                    },
                    File {
                        filename: "src/detail.rs".into(),
                        content: "mod slice_utils;\r\n\r\nuse std::cmp::Ordering;\r\nuse std::str::FromStr;\r\nuse derivative::Derivative;\r\nuse strum_macros::{EnumString, FromRepr};\r\nuse thiserror::Error;\r\nuse crate::detail::slice_utils::group_by::ClGroupBy;\r\nuse crate::detail::slice_utils::SliceUtils;\r\n\r\n#[derive(Debug, Error)]\r\npub enum Error {\r\n    #[error(\"Invalid card format `{0}`\")]\r\n    InvalidCardFormat(String),\r\n    #[error(\"Invalid card count `{0}` (should be 5)\")]\r\n    InvalidCardCount(usize),\r\n}\r\n\r\ntrait OrInvalidCardFormat<T> {\r\n    fn or_invalid_card_format(self, card_format: &str) -> T;\r\n}\r\n\r\nimpl OrInvalidCardFormat<Error> for strum::ParseError {\r\n    fn or_invalid_card_format(self, card_format: &str) -> Error {\r\n        match self {\r\n            strum::ParseError::VariantNotFound => Error::InvalidCardFormat(card_format.to_string()),\r\n        }\r\n    }\r\n}\r\n\r\nimpl<T, E> OrInvalidCardFormat<Result<T, Error>> for Result<T, E>\r\n    where\r\n        E: OrInvalidCardFormat<Error>\r\n{\r\n    fn or_invalid_card_format(self, card_format: &str) -> Result<T, Error> {\r\n        self.map_err(|e| e.or_invalid_card_format(card_format))\r\n    }\r\n}\r\n\r\n#[repr(i8)]\r\n#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumString, FromRepr)]\r\nenum CardValue {\r\n    #[strum(serialize = \"2\")]\r\n    Two,\r\n    #[strum(serialize = \"3\")]\r\n    Three,\r\n    #[strum(serialize = \"4\")]\r\n    Four,\r\n    #[strum(serialize = \"5\")]\r\n    Five,\r\n    #[strum(serialize = \"6\")]\r\n    Six,\r\n    #[strum(serialize = \"7\")]\r\n    Seven,\r\n    #[strum(serialize = \"8\")]\r\n    Eight,\r\n    #[strum(serialize = \"9\")]\r\n    Nine,\r\n    #[strum(serialize = \"10\")]\r\n    Ten,\r\n    #[strum(serialize = \"J\")]\r\n    Jack,\r\n    #[strum(serialize = \"Q\")]\r\n    Queen,\r\n    #[strum(serialize = \"K\")]\r\n    King,\r\n    #[strum(serialize = \"A\")]\r\n    Ace,\r\n}\r\n\r\nimpl CardValue {\r\n    pub fn next(&self) -> Self {\r\n        match Self::from_repr((*self as i8) + 1) {\r\n            Some(cv) => cv,\r\n            None => Self::from_repr(0).unwrap(),\r\n        }\r\n    }\r\n}\r\n\r\n#[derive(Copy, Clone, PartialEq, Eq, EnumString)]\r\nenum Suit {\r\n    #[strum(serialize = \"H\")]\r\n    Hearts,\r\n    #[strum(serialize = \"S\")]\r\n    Spades,\r\n    #[strum(serialize = \"D\")]\r\n    Diamonds,\r\n    #[strum(serialize = \"C\")]\r\n    Clubs,\r\n}\r\n\r\n#[derive(Derivative)]\r\n#[derivative(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]\r\nstruct Card {\r\n    value: CardValue,\r\n    #[derivative(PartialEq = \"ignore\")]\r\n    #[derivative(PartialOrd = \"ignore\")]\r\n    #[derivative(Ord = \"ignore\")]\r\n    suit: Suit,\r\n}\r\n\r\nimpl Card {\r\n    pub fn new(card_s: &str) -> Result<Card, Error> {\r\n        let (value_s, suit_s) = card_s.split_at(card_s.len() - 1);\r\n        let value = CardValue::from_str(value_s).or_invalid_card_format(card_s)?;\r\n        let suit = Suit::from_str(suit_s).or_invalid_card_format(card_s)?;\r\n        Ok(Card { value, suit })\r\n    }\r\n}\r\n\r\n#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]\r\nenum HandType {\r\n    HighCard,\r\n    Pair,\r\n    TwoPairs,\r\n    ThreeOfAKind,\r\n    Straight,\r\n    Flush,\r\n    FullHouse,\r\n    FourOfAKind,\r\n    StraightFlush,\r\n}\r\n\r\n#[derive(Derivative)]\r\n#[derivative(PartialEq, Eq, PartialOrd, Ord)]\r\npub struct Hand<'a> {\r\n    #[derivative(PartialEq = \"ignore\")]\r\n    #[derivative(PartialOrd = \"ignore\")]\r\n    #[derivative(Ord = \"ignore\")]\r\n    hand_s: &'a str,\r\n    hand_type: HandType,\r\n    contributing_cards: Vec<Card>,\r\n    non_contributing_cards: Vec<Card>,\r\n}\r\n\r\nimpl<'a> Hand<'a> {\r\n    pub fn new(hand_s: &'a str) -> Result<Hand<'a>, Error> {\r\n        let mut cards = hand_s.split(' ')\r\n            .map(|card_s| Card::new(card_s))\r\n            .collect::<Result<Vec<_>, _>>()?;\r\n        if cards.len() != 5 {\r\n            return Err(Error::InvalidCardCount(cards.len()))\r\n        }\r\n        cards.sort_unstable();\r\n\r\n        let cards = &cards[..];\r\n        let hand_type: Option<_>;\r\n        let mut contributing_cards: Option<Vec<_>> = None;\r\n        let mut non_contributing_cards: Option<Vec<_>> = None;\r\n\r\n        if Self::is_straight(&cards) {\r\n            if Self::is_flush(&cards) {\r\n                hand_type = Some(HandType::StraightFlush);\r\n            } else {\r\n                hand_type = Some(HandType::Straight);\r\n            }\r\n\r\n            if Self::is_low_straight(&cards) {\r\n                let mut cc = Vec::with_capacity(5);\r\n                cc.push(cards[4]);\r\n                cc.append(&mut cards[0..=3].into());\r\n                contributing_cards = Some(cc);\r\n            }\r\n        } else if Self::is_flush(&cards) {\r\n            hand_type = Some(HandType::Flush);\r\n        } else if Self::is_full_house(&cards) {\r\n            hand_type = Some(HandType::FullHouse);\r\n            if cards[1] == cards[2] {\r\n                contributing_cards = Some(cards[0..=2].into());\r\n                non_contributing_cards = Some(cards[3..=4].into());\r\n            } else {\r\n                contributing_cards = Some(cards[2..=4].into());\r\n                non_contributing_cards = Some(cards[0..=1].into());\r\n            }\r\n        } else {\r\n            // Once https://doc.rust-lang.org/std/primitive.slice.html#method.group_by\r\n            // is stabilized, we could switch to that instead.\r\n            let mut chunks: Vec<_> = cards.cl_group_by(|a, b| a == b).collect();\r\n            chunks.sort_unstable_by(|&c1, &c2| {\r\n                match c1.len().cmp(&c2.len()) {\r\n                    Ordering::Equal => c1[0].cmp(&c2[0]),\r\n                    cmp => cmp,\r\n                }\r\n            });\r\n\r\n            hand_type = match chunks.last().unwrap().len() {\r\n                4 => Some(HandType::FourOfAKind),\r\n                3 => Some(HandType::ThreeOfAKind),\r\n                2 => match chunks.iter().filter(|&c| c.len() == 2).count() {\r\n                    2 => {\r\n                        contributing_cards = Some(chunks[(chunks.len() - 2)..chunks.len()].to_flattened_vec());\r\n                        non_contributing_cards = Some(chunks[0].into());\r\n                        Some(HandType::TwoPairs)\r\n                    },\r\n                    _ => Some(HandType::Pair),\r\n                },\r\n                _ => Some(HandType::HighCard),\r\n            };\r\n\r\n            contributing_cards = contributing_cards.or_else(|| Some(chunks.last().unwrap().to_vec()));\r\n            non_contributing_cards = non_contributing_cards.or_else(|| Some(chunks[0..(chunks.len() - 1)].to_flattened_vec()));\r\n        }\r\n\r\n        let hand_type = hand_type.unwrap();\r\n        let mut contributing_cards = contributing_cards.unwrap_or_else(|| cards.into());\r\n        let mut non_contributing_cards = non_contributing_cards.unwrap_or_else(|| Vec::new());\r\n        contributing_cards.reverse();\r\n        non_contributing_cards.reverse();\r\n\r\n        Ok(Hand {\r\n            hand_s,\r\n            hand_type,\r\n            contributing_cards,\r\n            non_contributing_cards,\r\n        })\r\n    }\r\n\r\n    pub fn hand_s(&self) -> &'a str {\r\n        self.hand_s\r\n    }\r\n\r\n    fn is_flush(cards: &[Card]) -> bool {\r\n        cards.iter().all(|c| c.suit == cards[0].suit)\r\n    }\r\n\r\n    fn is_straight(cards: &[Card]) -> bool {\r\n        cards.iter().skip(1).enumerate().all(|(i, c)| {\r\n            cards[i].value.next() == c.value ||\r\n                (i == 3 && cards[i].value == CardValue::Five && c.value == CardValue::Ace)\r\n        })\r\n    }\r\n\r\n    fn is_low_straight(cards: &[Card]) -> bool {\r\n        Self::is_straight(cards) && cards[3].value == CardValue::Five && cards[4].value == CardValue::Ace\r\n    }\r\n\r\n    fn is_full_house(cards: &[Card]) -> bool {\r\n        (cards[0] == cards[1] && cards[2] == cards[3] && cards[3] == cards[4] && cards[1] != cards[2]) ||\r\n            (cards[0] == cards[1] && cards[1] == cards[2] && cards[3] == cards[4] && cards[2] != cards[3])\r\n    }\r\n}\r\n".into(),
                        digest: "cdfc3584877237d5f157091b6e756254aec056f1".into(),
                    },
                    File {
                        filename: "src/detail/slice_utils.rs".into(),
                        content: "pub mod group_by;\r\n\r\npub trait SliceUtils<T> {\r\n    fn to_flattened_vec(&self) -> Vec<T>;\r\n}\r\n\r\nimpl<T: Copy> SliceUtils<T> for [&[T]] {\r\n    fn to_flattened_vec(&self) -> Vec<T> {\r\n        self.iter().map(|&c| c.to_vec()).flatten().collect()\r\n    }\r\n}\r\n".into(),
                        digest: "2bdcb3f11084cd7d082ff08efdda21dfb50fca5a".into(),
                    },
                    File {
                        filename: "src/detail/slice_utils/group_by.rs".into(),
                        content: "pub struct GroupBy<'a, T: 'a, F: FnMut(&T, &T) -> bool> {\r\n    s: &'a [T],\r\n    len: usize,\r\n    i: usize,\r\n    f: F,\r\n}\r\n\r\npub trait ClGroupBy<'a, T: 'a> {\r\n    fn cl_group_by<F: FnMut(&T, &T) -> bool>(&self, f: F) -> GroupBy<'a, T, F>;\r\n}\r\n\r\nimpl<'a, T: 'a> ClGroupBy<'a, T> for &'a [T] {\r\n    fn cl_group_by<F: FnMut(&T, &T) -> bool>(&self, f: F) -> GroupBy<'a, T, F> {\r\n        GroupBy::new(self, f)\r\n    }\r\n}\r\n\r\nimpl<'a, T: 'a, F: FnMut(&T, &T) -> bool> GroupBy<'a, T, F> {\r\n    fn new(s: &'a [T], f: F) -> Self {\r\n        GroupBy {\r\n            s,\r\n            len: s.len(),\r\n            i: 0,\r\n            f,\r\n        }\r\n    }\r\n}\r\n\r\nimpl<'a, T: 'a, F: FnMut(&T, &T) -> bool> Iterator for GroupBy<'a, T, F> {\r\n    type Item = &'a [T];\r\n\r\n    fn next(&mut self) -> Option<Self::Item> {\r\n        match self.i {\r\n            end if end == self.len => None,\r\n            beg => {\r\n                let cur = &self.s[beg];\r\n                self.i += 1;\r\n                while self.i < self.len && (self.f)(cur, &self.s[self.i]) {\r\n                    self.i += 1;\r\n                }\r\n                Some(&self.s[beg..self.i])\r\n            }\r\n        }\r\n    }\r\n}\r\n\r\nimpl<'a, T: 'a, F: FnMut(&T, &T) -> bool> ::std::iter::FusedIterator for GroupBy<'a, T, F> {}\r\n".into(),
                        digest: "b332b7e8998e9711acce79b8c3b1f65610d2cf99".into(),
                    },
                    File {
                        filename: "Cargo.toml".into(),
                        content: "[package]\nedition = \"2021\"\nname = \"poker\"\nversion = \"1.1.0\"\n\n[dependencies]\nderivative = \"2.2.0\"\nstrum = \"0.24.1\"\nstrum_macros = \"0.24.3\"\nthiserror = \"1.0.40\"\n".into(),
                        digest: "9ad1c8abd08fcc3111eaf728a9fb1f3717d10ad8".into(),
                    }
                ]
            };
            let actual: files::Response = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}

mod file {
    mod deserialize {
        use mini_exercism::api::v2::submission::files::File;

        #[test]
        fn test_all() {
            let json = r#"{
                "filename": "Cargo.toml",
                "content": "[package]\nedition = \"2021\"\nname = \"poker\"\nversion = \"1.1.0\"\n\n[dependencies]\nderivative = \"2.2.0\"\nstrum = \"0.24.1\"\nstrum_macros = \"0.24.3\"\nthiserror = \"1.0.40\"\n",
                "digest": "9ad1c8abd08fcc3111eaf728a9fb1f3717d10ad8"
            }"#;

            let expected = File {
                filename: "Cargo.toml".into(),
                content: "[package]\nedition = \"2021\"\nname = \"poker\"\nversion = \"1.1.0\"\n\n[dependencies]\nderivative = \"2.2.0\"\nstrum = \"0.24.1\"\nstrum_macros = \"0.24.3\"\nthiserror = \"1.0.40\"\n".into(),
                digest: "9ad1c8abd08fcc3111eaf728a9fb1f3717d10ad8".into(),
            };
            let actual: File = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
