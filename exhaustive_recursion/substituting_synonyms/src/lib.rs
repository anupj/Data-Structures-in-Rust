use std::collections::HashMap;

/// Takes in a sentence and an object as arguments. The
/// object contains words as keys whose values are arrays
/// containing synonyms. The function should return
/// an array containing all possible sentences that can be
/// formed by substituting words of the sentence with their
/// synonyms.
///
/// You may return the possible sentences in any order,
/// as long as you return all of them.
fn substitute_synonyms(sentence: &str, synonyms: &HashMap<String, Vec<String>>) -> Vec<String> {
    let words: Vec<&str> = sentence.split(' ').collect();
    let arrays = generate(&words, synonyms);
    arrays
        .into_iter()
        .map(|subarray| subarray.join(" "))
        .collect()
}

fn generate(words: &[&str], synonyms: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    if words.is_empty() {
        return vec![vec![]];
    }

    let first_word = words[0];
    let remaining_words = &words[1..];
    if let Some(synonyms_for_first_word) = synonyms.get(first_word) {
        let mut result = vec![];
        let subarrays = generate(remaining_words, synonyms);
        for synonym in synonyms_for_first_word {
            let mut subarray_with_synonym = subarrays.clone();
            for subarray in &mut subarray_with_synonym {
                subarray.insert(0, synonym.clone());
            }
            result.append(&mut subarray_with_synonym);
        }
        result
    } else {
        let subarrays = generate(remaining_words, synonyms);
        subarrays
            .into_iter()
            .map(|mut subarray| {
                subarray.insert(0, first_word.to_string());
                subarray
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitute_synonyms_00() {
        let sentence = "follow the yellow brick road";
        let mut synonyms: HashMap<String, Vec<String>> = HashMap::new();
        synonyms.insert(
            "follow".to_string(),
            vec!["chase".to_string(), "pursue".to_string()],
        );
        synonyms.insert(
            "yellow".to_string(),
            vec!["gold".to_string(), "amber".to_string(), "lemon".to_string()],
        );
        let result = substitute_synonyms(sentence, &synonyms);
        let expected = vec![
            "chase the gold brick road",
            "chase the amber brick road",
            "chase the lemon brick road",
            "pursue the gold brick road",
            "pursue the amber brick road",
            "pursue the lemon brick road",
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn substitute_synonyms_01() {
        let sentence = "I think it's gonna be a long long time";
        let mut synonyms: HashMap<String, Vec<String>> = HashMap::new();
        synonyms.insert(
            "think".to_string(),
            vec!["believe".to_string(), "reckon".to_string()],
        );
        synonyms.insert(
            "long".to_string(),
            vec!["lengthy".to_string(), "prolonged".to_string()],
        );
        let result = substitute_synonyms(sentence, &synonyms);
        let expected = vec![
            "I believe it's gonna be a lengthy lengthy time",
            "I believe it's gonna be a lengthy prolonged time",
            "I believe it's gonna be a prolonged lengthy time",
            "I believe it's gonna be a prolonged prolonged time",
            "I reckon it's gonna be a lengthy lengthy time",
            "I reckon it's gonna be a lengthy prolonged time",
            "I reckon it's gonna be a prolonged lengthy time",
            "I reckon it's gonna be a prolonged prolonged time",
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn substitute_synonyms_02() {
        let sentence = "palms sweaty knees weak arms heavy";
        let mut synonyms: HashMap<String, Vec<String>> = HashMap::new();
        synonyms.insert(
            "palms".to_string(),
            vec!["hands".to_string(), "fists".to_string()],
        );
        synonyms.insert(
            "heavy".to_string(),
            vec![
                "weighty".to_string(),
                "hefty".to_string(),
                "burdensome".to_string(),
            ],
        );
        synonyms.insert(
            "weak".to_string(),
            vec![
                "fragile".to_string(),
                "feeble".to_string(),
                "frail".to_string(),
                "sickly".to_string(),
            ],
        );
        let result = substitute_synonyms(sentence, &synonyms);
        let expected = vec![
            "hands sweaty knees fragile arms weighty",
            "hands sweaty knees fragile arms hefty",
            "hands sweaty knees fragile arms burdensome",
            "hands sweaty knees feeble arms weighty",
            "hands sweaty knees feeble arms hefty",
            "hands sweaty knees feeble arms burdensome",
            "hands sweaty knees frail arms weighty",
            "hands sweaty knees frail arms hefty",
            "hands sweaty knees frail arms burdensome",
            "hands sweaty knees sickly arms weighty",
            "hands sweaty knees sickly arms hefty",
            "hands sweaty knees sickly arms burdensome",
            "fists sweaty knees fragile arms weighty",
            "fists sweaty knees fragile arms hefty",
            "fists sweaty knees fragile arms burdensome",
            "fists sweaty knees feeble arms weighty",
            "fists sweaty knees feeble arms hefty",
            "fists sweaty knees feeble arms burdensome",
            "fists sweaty knees frail arms weighty",
            "fists sweaty knees frail arms hefty",
            "fists sweaty knees frail arms burdensome",
            "fists sweaty knees sickly arms weighty",
            "fists sweaty knees sickly arms hefty",
            "fists sweaty knees sickly arms burdensome",
        ];

        assert_eq!(result, expected);
    }
}
