use anyhow::Result;
use game_quest_parser_Hodik::Parser;

#[test]
fn test_parse_full_quest() -> Result<()> {
    let input = r#"
        quest "Main Quest" {
            active: true,
            reward: 100,
            step: "Start"
        }
    "#;
    let mut parser = Parser::new(input)?;
    let quest = parser.parse_quest()?;
    assert_eq!(quest.name, "Main Quest");
    Ok(())
}

#[test]
fn test_grammar_rule_active_boolean() -> Result<()> {
    let input = r#"quest "Test" { active: false }"#;
    let mut parser = Parser::new(input)?;
    let quest = parser.parse_quest()?;
    assert_eq!(quest.active, false);
    Ok(())
}

#[test]
fn test_grammar_rule_reward_number() -> Result<()> {
    let input = r#"quest "Test" { reward: 999 }"#;
    let mut parser = Parser::new(input)?;
    let quest = parser.parse_quest()?;
    assert_eq!(quest.reward, 999);
    Ok(())
}

#[test]
fn test_grammar_rule_steps_list() -> Result<()> {
    let input = r#"quest "Test" { step: "A", step: "B" }"#;
    let mut parser = Parser::new(input)?;
    let quest = parser.parse_quest()?;
    assert_eq!(quest.steps.len(), 2);
    assert_eq!(quest.steps[0], "A");
    assert_eq!(quest.steps[1], "B");
    Ok(())
}

#[test]
fn test_grammar_error_missing_brace() {
    let input = r#"quest "Error" active: true"#;
    let mut parser = Parser::new(input).unwrap();
    assert!(parser.parse_quest().is_err());
}
