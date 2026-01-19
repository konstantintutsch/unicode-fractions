pub trait Numbers {
    fn to_superscript(&self) -> Self;
    fn to_subscript(&self) -> Self;
}
impl Numbers for String {
    fn to_superscript(&self) -> Self {
        let mut superscript = String::new();

        for c in self.chars() {
            let t = match c {
                '0' => '⁰',
                '1' => '¹',
                '2' => '²',
                '3' => '³',
                '4' => '⁴',
                '5' => '⁵',
                '6' => '⁶',
                '7' => '⁷',
                '8' => '⁸',
                '9' => '⁹',
                _ => c
            };

            superscript.push(t);
        }

        return superscript;
    } 

    fn to_subscript(&self) -> Self {
        let mut subscript = String::new();

        for c in self.chars() {
            let t = match c {
                '0' => '₀',
                '1' => '₁',
                '2' => '₂',
                '3' => '₃',
                '4' => '₄',
                '5' => '₅',
                '6' => '₆',
                '7' => '₇',
                '8' => '₈',
                '9' => '₉',
                _ => c
            };

            subscript.push(t);
        }

        return subscript;
    } 
}


