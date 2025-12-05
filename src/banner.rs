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
    ║                          ╔══════════════╗                         ║
    ║                          ║     + +      ║                         ║
    ║                          ╚══════════════╝                         ║
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
    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════════╗".bright_yellow());
    println!("{}", "║                    AUXILIUM (Help)                        ║".bright_yellow());
    println!("{}", "╠═══════════════════════════════════════════════════════════╣".bright_yellow());
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "║  DECLARATIONES (Declarations):                            ║".bright_yellow());
    println!("{}  {}",
        "║".bright_yellow(),
        "DECLARA X EST 42      - Declara variable X cum valore 42".white()
    );
    println!("{}  {}",
        "║".bright_yellow(),
        "DECLARA Y EST XIV     - Declara Y cum numero Romano XIV".white()
    );
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "║  ASSIGNATIONES (Assignments):                             ║".bright_yellow());
    println!("{}  {}",
        "║".bright_yellow(),
        "X EST 100             - Assigna 100 ad X".white()
    );
    println!("{}  {}",
        "║".bright_yellow(),
        "X EST X ADDIUS Y      - Assigna X + Y ad X".white()
    );
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "║  OPERATORES (Operators):                                  ║".bright_yellow());
    println!("{}  {}",
        "║".bright_yellow(),
        "ADDIUS      (+)  -  Additio".cyan()
    );
    println!("{}  {}",
        "║".bright_yellow(),
        "SUBTRAHE    (-)  -  Subtractio".cyan()
    );
    println!("{}  {}",
        "║".bright_yellow(),
        "MULTIPLICA  (*)  -  Multiplicatio".cyan()
    );
    println!("{}  {}",
        "║".bright_yellow(),
        "DIVIDE      (/)  -  Divisio".cyan()
    );
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "║  OUTPUT (SCRIBE):                                         ║".bright_yellow());
    println!("{}  {}",
        "║".bright_yellow(),
        "SCRIBE(\"Valor: {X}\")           - Imprime in Romanis".white()
    );
    println!("{}  {}",
        "║".bright_yellow(),
        "SCRIBE(\"Valor: {X}\", ARABIZA(X))  - Imprime in Arabicis".white()
    );
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "║  CEREMONIALE:                                             ║".bright_yellow());
    println!("{}  {}",
        "║".bright_yellow(),
        "AVTEM                 - Ceremoniale no-op (pure swag)".magenta()
    );
    println!("{}  {}",
        "║".bright_yellow(),
        "NOTA: commentarius    - Commentarius (ignoratur)".magenta()
    );
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "║  EXEMPLUM:                                                ║".bright_yellow());
    println!("{}  {}",
        "║".bright_yellow(),
        "DECLARA A EST XV".green()
    );
    println!("{}  {}",
        "║".bright_yellow(),
        "DECLARA B EST 10".green()
    );
    println!("{}  {}",
        "║".bright_yellow(),
        "DECLARA C EST A ADDIUS B".green()
    );
    println!("{}  {}",
        "║".bright_yellow(),
        "SCRIBE(\"Summa: {C}\")".green()
    );
    println!("{}", "║                                                           ║".bright_yellow());
    println!("{}", "╚═══════════════════════════════════════════════════════════╝".bright_yellow());
    println!();
}

/// Print farewell message
pub fn print_farewell() {
    println!();
    println!("{}", "    ╔═══════════════════════════════════════════╗".bright_yellow());
    println!("{}", "    ║                                           ║".bright_yellow());
    println!("    {}  {}  {}",
        "║".bright_yellow(),
        "VALE! (Farewell, noble programmer!)".bright_green().bold(),
        "║".bright_yellow()
    );
    println!("{}", "    ║                                           ║".bright_yellow());
    println!("{}", "    ║         Gloria Romae in perpetuum!        ║".bright_yellow());
    println!("{}", "    ║                                           ║".bright_yellow());
    println!("{}", "    ╚═══════════════════════════════════════════╝".bright_yellow());
    println!();
}
