use unicode_segmentation::UnicodeSegmentation;

pub fn string_value_types() {
    let hello: String = String::from("नमस्ते");

    //Bytes
    //[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // Scalar values
    // ['न', 'म', 'स', '्', 'त', 'े']

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Graphene Clusters
    // ["न", "म", "स्", "ते"]

    for g in "नमस्ते".graphemes(true) {
        println!("{}", g);
    }
}
