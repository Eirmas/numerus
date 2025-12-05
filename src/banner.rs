use colored::*;

/// Print the glorious Numerus++ startup banner
pub fn print_banner() {
    let banner = r#"
    ╔═══════════════════════════════════════════════════════════════════╗
    ║                                                                   ║
    ║   ███╗   ██╗██╗   ██╗███╗   ███╗███████╗██████╗ ██╗   ██╗███████╗ ║
    ║   ████╗  ██║██║   ██║████╗ ████║██╔════╝██╔══██╗██║   ██║██╔════╝ ║
    ║   ██╔██╗ ██║██║   ██║██╔████╔██║█████╗  ██████╔╝██║   ██║███████╗ ║
    ║   ██║╚██╗██║██║   ██║██║╚██╔╝██║██╔══╝  ██╔══██╗██║   ██║╚════██║ ║
    ║   ██║ ╚████║╚██████╔╝██║ ╚═╝ ██║███████╗██║  ██║╚██████╔╝███████║ ║
    ║   ╚═╝  ╚═══╝ ╚═════╝ ╚═╝     ╚═╝╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝ ║
    ║                          ╔═════════════╗                          ║
    ║                          ║     + +     ║                          ║
    ║                          ╚═════════════╝                          ║
    ║                                                                   ║
    ║             "Salve, Programmator! Roma Aeterna Est!"              ║
    ║                                                                   ║
    ║                        Anno Domini MMXXV                          ║
    ║                                                                   ║
    ╚═══════════════════════════════════════════════════════════════════╝
    "#;

    println!("{}", banner.bright_yellow().bold());
    println!("    {}",
        "  Scribe 'AUXILIUM' pro auxilio, 'EXITUS' pro exire.".bright_cyan()
    );
    println!();
}

/// Print a shorter version for when running files
pub fn print_mini_banner() {
    println!("{}", "═══════════════════════════════════════════".bright_yellow());
    println!("{} {} {}",
        "║".bright_yellow(),
        "NUMERUS++ INTERPRETATOREM".bright_white().bold(),
        "║".bright_yellow()
    );
    println!("{}", "═══════════════════════════════════════════".bright_yellow());
    println!();
}

/// Print help message
pub fn print_help() {
    let b = "║".bright_yellow();
    let w = 57; // inner width

    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════════╗".bright_yellow());
    println!("{}", "║                    AUXILIUM (Help)                        ║".bright_yellow());
    println!("{}", "╠═══════════════════════════════════════════════════════════╣".bright_yellow());
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "║ DECLARATIONES (Declarations):                             ║".bright_yellow());
    println!("{} {:<w$} {}", b, "DECLARA X EST 42      - Declara variable X cum valore 42".white(), b);
    println!("{} {:<w$} {}", b, "DECLARA Y EST XIV     - Declara Y cum numero Romano XIV".white(), b);
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "║ ASSIGNATIONES (Assignments):                              ║".bright_yellow());
    println!("{} {:<w$} {}", b, "X EST 100             - Assigna 100 ad X".white(), b);
    println!("{} {:<w$} {}", b, "X EST X ADDIUS Y      - Assigna X + Y ad X".white(), b);
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "║ OPERATORES (Operators):                                   ║".bright_yellow());
    println!("{} {:<w$} {}", b, "ADDIUS      (+)  -  Additio".cyan(), b);
    println!("{} {:<w$} {}", b, "SUBTRAHE    (-)  -  Subtractio".cyan(), b);
    println!("{} {:<w$} {}", b, "MULTIPLICA  (*)  -  Multiplicatio".cyan(), b);
    println!("{} {:<w$} {}", b, "DIVIDE      (/)  -  Divisio".cyan(), b);
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "║ OUTPUT (SCRIBE):                                          ║".bright_yellow());
    println!("{} {:<w$} {}", b, "SCRIBE(X)             - Imprime (numeri in Romanis)".white(), b);
    println!("{} {:<w$} {}", b, "SCRIBE(\"Salve!\")      - Imprime string".white(), b);
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "║ FUNCTIONES (Functions):                                   ║".bright_yellow());
    println!("{} {:<w$} {}", b, "ROMANIZA(42)          - Converte ad Roman string".cyan(), b);
    println!("{} {:<w$} {}", b, "ARABIZA(XLII)         - Converte ad Arabic string".cyan(), b);
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "║ CEREMONIALE:                                              ║".bright_yellow());
    println!("{} {:<w$} {}", b, "AVTEM                 - Ceremoniale no-op".magenta(), b);
    println!("{} {:<w$} {}", b, "NOTA: commentarius    - Commentarius (ignoratur)".magenta(), b);
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "║ EXEMPLUM:                                                 ║".bright_yellow());
    println!("{} {:<w$} {}", b, "DECLARA A EST XV".green(), b);
    println!("{} {:<w$} {}", b, "DECLARA B EST 10".green(), b);
    println!("{} {:<w$} {}", b, "DECLARA C EST A ADDIUS B".green(), b);
    println!("{} {:<w$} {}", b, "SCRIBE(\"Summa: \" ADDIUS C)".green(), b);
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "╚═══════════════════════════════════════════════════════════╝".bright_yellow());
    println!();
}

/// Print farewell message
pub fn print_farewell() {
    println!();
    println!("{}", "    ╔═══════════════════════════════════════════╗".bright_yellow());
    println!("{}", "    ║                                           ║".bright_yellow());
    println!("    {} {} {}",
        "║".bright_yellow(),
        "   VALE! (Farewell, noble programmer!)   ".bright_green().bold(),
        "║".bright_yellow()
    );
    println!("{}", "    ║                                           ║".bright_yellow());
    println!("{}", "    ║         Gloria Romae in perpetuum!        ║".bright_yellow());
    println!("{}", "    ║                                           ║".bright_yellow());
    println!("{}", "    ╚═══════════════════════════════════════════╝".bright_yellow());
    println!();
}
