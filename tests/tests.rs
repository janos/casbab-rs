// Copyright (c) 2024, Janoš Guljaš <janos@resenje.org>
// All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use casbab;

struct Case<'a> {
    input: Vec<&'a str>,
    camel: &'a str,
    pascal: &'a str,
    snake: &'a str,
    camel_snake: &'a str,
    screaming_snake: &'a str,
    kebab: &'a str,
    camel_kebab: &'a str,
    screaming_kebab: &'a str,
    lower: &'a str,
    title: &'a str,
    screaming: &'a str,
}

#[test]
fn casbab_test() {
    let cases = vec![
        Case {
            input: vec![
                "camelSnakeKebab",
                "CamelSnakeKebab",
                "camel_snake_kebab",
                "Camel_Snake_Kebab",
                "CAMEL_SNAKE_KEBAB",
                "camel-snake-kebab",
                "Camel-Snake-Kebab",
                "CAMEL-SNAKE-KEBAB",
                "camel snake kebab",
                "Camel Snake Kebab",
                "CAMEL SNAKE KEBAB",
                "camel__snake_kebab",
                "camel___snake_kebab",
                "camel____snake_kebab",
                "camel_ snake_kebab",
                "camel_  snake_kebab",
                "camel_   snake_kebab",
                "camel_-snake_kebab",
                "camel_ -snake_kebab",
                "camel - snake_kebab",
                " camel - snake_kebab",
                "CAMELSnakeKebab",
                "camelSNAKEKebab   ",
                "   camelSnakeKEBAB",
            ],
            camel: "camelSnakeKebab",
            pascal: "CamelSnakeKebab",
            snake: "camel_snake_kebab",
            camel_snake: "Camel_Snake_Kebab",
            screaming_snake: "CAMEL_SNAKE_KEBAB",
            kebab: "camel-snake-kebab",
            camel_kebab: "Camel-Snake-Kebab",
            screaming_kebab: "CAMEL-SNAKE-KEBAB",
            lower: "camel snake kebab",
            title: "Camel Snake Kebab",
            screaming: "CAMEL SNAKE KEBAB",
        },
        Case {
            input: vec![
                "__camel_snake_kebab__",
                "__camel_snakeKEBAB__",
                "__ Camel-snakeKEBAB__",
            ],
            camel: "camelSnakeKebab",
            pascal: "CamelSnakeKebab",
            snake: "__camel_snake_kebab__",
            camel_snake: "__Camel_Snake_Kebab__",
            screaming_snake: "__CAMEL_SNAKE_KEBAB__",
            kebab: "camel-snake-kebab",
            camel_kebab: "Camel-Snake-Kebab",
            screaming_kebab: "CAMEL-SNAKE-KEBAB",
            lower: "camel snake kebab",
            title: "Camel Snake Kebab",
            screaming: "CAMEL SNAKE KEBAB",
        },
        Case {
            input: vec![
                "__ camel-snake_kebab__ _",
                "__ camelSnake_Kebab_",
                "__CamelSnake_Kebab_",
                "__CamelSNAKE_Kebab_",
            ],
            camel: "camelSnakeKebab",
            pascal: "CamelSnakeKebab",
            snake: "__camel_snake_kebab_",
            camel_snake: "__Camel_Snake_Kebab_",
            screaming_snake: "__CAMEL_SNAKE_KEBAB_",
            kebab: "camel-snake-kebab",
            camel_kebab: "Camel-Snake-Kebab",
            screaming_kebab: "CAMEL-SNAKE-KEBAB",
            lower: "camel snake kebab",
            title: "Camel Snake Kebab",
            screaming: "CAMEL SNAKE KEBAB",
        },
        Case {
            input: vec!["--camel-snake-kebab", "--CAMELSnake_kebab"],
            camel: "camelSnakeKebab",
            pascal: "CamelSnakeKebab",
            snake: "camel_snake_kebab",
            camel_snake: "Camel_Snake_Kebab",
            screaming_snake: "CAMEL_SNAKE_KEBAB",
            kebab: "--camel-snake-kebab",
            camel_kebab: "--Camel-Snake-Kebab",
            screaming_kebab: "--CAMEL-SNAKE-KEBAB",
            lower: "camel snake kebab",
            title: "Camel Snake Kebab",
            screaming: "CAMEL SNAKE KEBAB",
        },
        Case {
            input: vec!["-camel-snake-kebab----", "-CAMEL   Snake_kebab ----"],
            camel: "camelSnakeKebab",
            pascal: "CamelSnakeKebab",
            snake: "camel_snake_kebab",
            camel_snake: "Camel_Snake_Kebab",
            screaming_snake: "CAMEL_SNAKE_KEBAB",
            kebab: "-camel-snake-kebab----",
            camel_kebab: "-Camel-Snake-Kebab----",
            screaming_kebab: "-CAMEL-SNAKE-KEBAB----",
            lower: "camel snake kebab",
            title: "Camel Snake Kebab",
            screaming: "CAMEL SNAKE KEBAB",
        },
        Case {
            input: vec![
                "xCamelXXSnakeXXXKebab",
                "XCamelXXSnakeXXXKebab",
                "x_camel_xx_snake_xxx_kebab",
                "X_Camel_XX_Snake_XXX_Kebab",
                "X_CAMEL_XX_SNAKE_XXX_KEBAB",
                "x-camel-xx-snake-xxx-kebab",
                "X-Camel-XX_Snake-XXX-Kebab",
                "X-CAMEL-XX_SNAKE-XXX-KEBAB",
                "x camel xx snake xxx kebab",
                "X Camel XX Snake XXX Kebab",
                "X CAMEL XX SNAKE XXX KEBAB",
            ],
            camel: "xCamelXxSnakeXxxKebab",
            pascal: "XCamelXxSnakeXxxKebab",
            snake: "x_camel_xx_snake_xxx_kebab",
            camel_snake: "X_Camel_Xx_Snake_Xxx_Kebab",
            screaming_snake: "X_CAMEL_XX_SNAKE_XXX_KEBAB",
            kebab: "x-camel-xx-snake-xxx-kebab",
            camel_kebab: "X-Camel-Xx-Snake-Xxx-Kebab",
            screaming_kebab: "X-CAMEL-XX-SNAKE-XXX-KEBAB",
            lower: "x camel xx snake xxx kebab",
            title: "X Camel Xx Snake Xxx Kebab",
            screaming: "X CAMEL XX SNAKE XXX KEBAB",
        },
        Case {
            input: vec![
                "Ово је Brave NewСвет",
                " Ово је Brave NewСвет",
                " Ово је Brave NewСвет    ",
            ],
            camel: "овоЈеBraveNewСвет",
            pascal: "ОвоЈеBraveNewСвет",
            snake: "ово_је_brave_new_свет",
            camel_snake: "Ово_Је_Brave_New_Свет",
            screaming_snake: "ОВО_ЈЕ_BRAVE_NEW_СВЕТ",
            kebab: "ово-је-brave-new-свет",
            camel_kebab: "Ово-Је-Brave-New-Свет",
            screaming_kebab: "ОВО-ЈЕ-BRAVE-NEW-СВЕТ",
            lower: "ово је brave new свет",
            title: "Ово Је Brave New Свет",
            screaming: "ОВО ЈЕ BRAVE NEW СВЕТ",
        },
        Case {
            input: vec![""],
            camel: "",
            pascal: "",
            snake: "",
            camel_snake: "",
            screaming_snake: "",
            kebab: "",
            camel_kebab: "",
            screaming_kebab: "",
            lower: "",
            title: "",
            screaming: "",
        },
    ];

    for c in cases {
        for input in c.input {
            assert_eq!(casbab::camel(input), c.camel);
            assert_eq!(casbab::pascal(input), c.pascal);
            assert_eq!(casbab::snake(input), c.snake);
            assert_eq!(casbab::camel_snake(input), c.camel_snake);
            assert_eq!(casbab::screaming_snake(input), c.screaming_snake);
            assert_eq!(casbab::kebab(input), c.kebab);
            assert_eq!(casbab::camel_kebab(input), c.camel_kebab);
            assert_eq!(casbab::screaming_kebab(input), c.screaming_kebab);
            assert_eq!(casbab::lower(input), c.lower);
            assert_eq!(casbab::title(input), c.title);
            assert_eq!(casbab::screaming(input), c.screaming);
        }
    }
}
