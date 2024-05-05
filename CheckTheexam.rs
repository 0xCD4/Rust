fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    let mut score = 0;

    for (correct, student) in arr_a.iter().zip(arr_b.iter()) {
        if student.is_empty() {
            // Blank answer
            score += 0;
        } else if student == correct {
            score += 4;
        } else {
            score -= 1;
        }
    }
    if score < 0 {
        return 0;
    }

    score
}
