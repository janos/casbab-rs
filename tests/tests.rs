// Copyright (c) 2024, Janoš Guljaš <janos@resenje.org>
// All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use casbab;

struct Case {
    input: Vec<String>,
    camel: String,
    pascal: String,
    snake: String,
    camel_snake: String,
    screaming_snake: String,
    kebab: String,
    camel_kebab: String,
    screaming_kebab: String,
    lower: String,
    title: String,
    screaming: String,
}

#[test]
fn casbab_test() {
    let cases = vec![
        Case {
            input: vec![
                "camelSnakeKebab".to_string(),
                "CamelSnakeKebab".to_string(),
                "camel_snake_kebab".to_string(),
                "Camel_Snake_Kebab".to_string(),
                "CAMEL_SNAKE_KEBAB".to_string(),
                "camel-snake-kebab".to_string(),
                "Camel-Snake-Kebab".to_string(),
                "CAMEL-SNAKE-KEBAB".to_string(),
                "camel snake kebab".to_string(),
                "Camel Snake Kebab".to_string(),
                "CAMEL SNAKE KEBAB".to_string(),
                "camel__snake_kebab".to_string(),
                "camel___snake_kebab".to_string(),
                "camel____snake_kebab".to_string(),
                "camel_ snake_kebab".to_string(),
                "camel_  snake_kebab".to_string(),
                "camel_   snake_kebab".to_string(),
                "camel_-snake_kebab".to_string(),
                "camel_ -snake_kebab".to_string(),
                "camel - snake_kebab".to_string(),
                " camel - snake_kebab".to_string(),
                "CAMELSnakeKebab".to_string(),
                "camelSNAKEKebab   ".to_string(),
                "   camelSnakeKEBAB".to_string(),
            ],
            camel: "camelSnakeKebab".to_string(),
            pascal: "CamelSnakeKebab".to_string(),
            snake: "camel_snake_kebab".to_string(),
            camel_snake: "Camel_Snake_Kebab".to_string(),
            screaming_snake: "CAMEL_SNAKE_KEBAB".to_string(),
            kebab: "camel-snake-kebab".to_string(),
            camel_kebab: "Camel-Snake-Kebab".to_string(),
            screaming_kebab: "CAMEL-SNAKE-KEBAB".to_string(),
            lower: "camel snake kebab".to_string(),
            title: "Camel Snake Kebab".to_string(),
            screaming: "CAMEL SNAKE KEBAB".to_string(),
        },
        Case {
            input: vec![
                "__camel_snake_kebab__".to_string(),
                "__camel_snakeKEBAB__".to_string(),
                "__ Camel-snakeKEBAB__".to_string(),
            ],
            camel: "camelSnakeKebab".to_string(),
            pascal: "CamelSnakeKebab".to_string(),
            snake: "__camel_snake_kebab__".to_string(),
            camel_snake: "__Camel_Snake_Kebab__".to_string(),
            screaming_snake: "__CAMEL_SNAKE_KEBAB__".to_string(),
            kebab: "camel-snake-kebab".to_string(),
            camel_kebab: "Camel-Snake-Kebab".to_string(),
            screaming_kebab: "CAMEL-SNAKE-KEBAB".to_string(),
            lower: "camel snake kebab".to_string(),
            title: "Camel Snake Kebab".to_string(),
            screaming: "CAMEL SNAKE KEBAB".to_string(),
        },
        Case {
            input: vec![
                "__ camel-snake_kebab__ _".to_string(),
                "__ camelSnake_Kebab_".to_string(),
                "__CamelSnake_Kebab_".to_string(),
                "__CamelSNAKE_Kebab_".to_string(),
            ],
            camel: "camelSnakeKebab".to_string(),
            pascal: "CamelSnakeKebab".to_string(),
            snake: "__camel_snake_kebab_".to_string(),
            camel_snake: "__Camel_Snake_Kebab_".to_string(),
            screaming_snake: "__CAMEL_SNAKE_KEBAB_".to_string(),
            kebab: "camel-snake-kebab".to_string(),
            camel_kebab: "Camel-Snake-Kebab".to_string(),
            screaming_kebab: "CAMEL-SNAKE-KEBAB".to_string(),
            lower: "camel snake kebab".to_string(),
            title: "Camel Snake Kebab".to_string(),
            screaming: "CAMEL SNAKE KEBAB".to_string(),
        },
        Case {
            input: vec![
                "--camel-snake-kebab".to_string(),
                "--CAMELSnake_kebab".to_string(),
            ],
            camel: "camelSnakeKebab".to_string(),
            pascal: "CamelSnakeKebab".to_string(),
            snake: "camel_snake_kebab".to_string(),
            camel_snake: "Camel_Snake_Kebab".to_string(),
            screaming_snake: "CAMEL_SNAKE_KEBAB".to_string(),
            kebab: "--camel-snake-kebab".to_string(),
            camel_kebab: "--Camel-Snake-Kebab".to_string(),
            screaming_kebab: "--CAMEL-SNAKE-KEBAB".to_string(),
            lower: "camel snake kebab".to_string(),
            title: "Camel Snake Kebab".to_string(),
            screaming: "CAMEL SNAKE KEBAB".to_string(),
        },
        Case {
            input: vec![
                "-camel-snake-kebab----".to_string(),
                "-CAMEL   Snake_kebab ----".to_string(),
            ],
            camel: "camelSnakeKebab".to_string(),
            pascal: "CamelSnakeKebab".to_string(),
            snake: "camel_snake_kebab".to_string(),
            camel_snake: "Camel_Snake_Kebab".to_string(),
            screaming_snake: "CAMEL_SNAKE_KEBAB".to_string(),
            kebab: "-camel-snake-kebab----".to_string(),
            camel_kebab: "-Camel-Snake-Kebab----".to_string(),
            screaming_kebab: "-CAMEL-SNAKE-KEBAB----".to_string(),
            lower: "camel snake kebab".to_string(),
            title: "Camel Snake Kebab".to_string(),
            screaming: "CAMEL SNAKE KEBAB".to_string(),
        },
        Case {
            input: vec![
                "xCamelXXSnakeXXXKebab".to_string(),
                "XCamelXXSnakeXXXKebab".to_string(),
                "x_camel_xx_snake_xxx_kebab".to_string(),
                "X_Camel_XX_Snake_XXX_Kebab".to_string(),
                "X_CAMEL_XX_SNAKE_XXX_KEBAB".to_string(),
                "x-camel-xx-snake-xxx-kebab".to_string(),
                "X-Camel-XX_Snake-XXX-Kebab".to_string(),
                "X-CAMEL-XX_SNAKE-XXX-KEBAB".to_string(),
                "x camel xx snake xxx kebab".to_string(),
                "X Camel XX Snake XXX Kebab".to_string(),
                "X CAMEL XX SNAKE XXX KEBAB".to_string(),
            ],
            camel: "xCamelXxSnakeXxxKebab".to_string(),
            pascal: "XCamelXxSnakeXxxKebab".to_string(),
            snake: "x_camel_xx_snake_xxx_kebab".to_string(),
            camel_snake: "X_Camel_Xx_Snake_Xxx_Kebab".to_string(),
            screaming_snake: "X_CAMEL_XX_SNAKE_XXX_KEBAB".to_string(),
            kebab: "x-camel-xx-snake-xxx-kebab".to_string(),
            camel_kebab: "X-Camel-Xx-Snake-Xxx-Kebab".to_string(),
            screaming_kebab: "X-CAMEL-XX-SNAKE-XXX-KEBAB".to_string(),
            lower: "x camel xx snake xxx kebab".to_string(),
            title: "X Camel Xx Snake Xxx Kebab".to_string(),
            screaming: "X CAMEL XX SNAKE XXX KEBAB".to_string(),
        },
        Case {
            input: vec![
                "Ово је Brave NewСвет".to_string(),
                " Ово је Brave NewСвет".to_string(),
                " Ово је Brave NewСвет    ".to_string(),
            ],
            camel: "овоЈеBraveNewСвет".to_string(),
            pascal: "ОвоЈеBraveNewСвет".to_string(),
            snake: "ово_је_brave_new_свет".to_string(),
            camel_snake: "Ово_Је_Brave_New_Свет".to_string(),
            screaming_snake: "ОВО_ЈЕ_BRAVE_NEW_СВЕТ".to_string(),
            kebab: "ово-је-brave-new-свет".to_string(),
            camel_kebab: "Ово-Је-Brave-New-Свет".to_string(),
            screaming_kebab: "ОВО-ЈЕ-BRAVE-NEW-СВЕТ".to_string(),
            lower: "ово је brave new свет".to_string(),
            title: "Ово Је Brave New Свет".to_string(),
            screaming: "ОВО ЈЕ BRAVE NEW СВЕТ".to_string(),
        },
        Case {
            input: vec!["".to_string(), " ".to_string(), "     ".to_string()],
            camel: "".to_string(),
            pascal: "".to_string(),
            snake: "".to_string(),
            camel_snake: "".to_string(),
            screaming_snake: "".to_string(),
            kebab: "".to_string(),
            camel_kebab: "".to_string(),
            screaming_kebab: "".to_string(),
            lower: "".to_string(),
            title: "".to_string(),
            screaming: "".to_string(),
        },
    ];

    for c in cases {
        for input in c.input {
            assert_eq!(casbab::camel(input.as_str()), c.camel);
            assert_eq!(casbab::pascal(input.as_str()), c.pascal);
            assert_eq!(casbab::snake(input.as_str()), c.snake);
            assert_eq!(casbab::camel_snake(input.as_str()), c.camel_snake);
            assert_eq!(casbab::screaming_snake(input.as_str()), c.screaming_snake);
            assert_eq!(casbab::kebab(input.as_str()), c.kebab);
            assert_eq!(casbab::camel_kebab(input.as_str()), c.camel_kebab);
            assert_eq!(casbab::screaming_kebab(input.as_str()), c.screaming_kebab);
            assert_eq!(casbab::lower(input.as_str()), c.lower);
            assert_eq!(casbab::title(input.as_str()), c.title);
            assert_eq!(casbab::screaming(input.as_str()), c.screaming);
        }
    }
}
