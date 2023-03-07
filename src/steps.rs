use lisudoku_solver::types::{SolutionStep, Rule};

pub fn compute_steps_text(steps: Vec<SolutionStep>) -> String {
  let relevant_steps: Vec<&SolutionStep> = steps.iter().filter(|step| step.rule != Rule::Candidates).collect();
  if relevant_steps.len() == 1 {
    return format!("There is a {}  \n\n", step_description(relevant_steps[0]))
  }

  let mut text = String::default();
  text += "Steps to make progress ";
  text += "^\\(I ^ignored ^any ^existing ^pencilmarks, ^the ^helpful ^steps ^are ^likely ^at ^the ^end ^of ^the ^list)  \n";
  text += "* Make sure to write in full [pencilmarks](https://www.sudopedia.org/wiki/Pencilmark)  \n";
  for (index, step) in relevant_steps.iter().enumerate() {
    text += "* ";
    if index == relevant_steps.len() - 1 {
      text += "Finally, there is a ";
    }
    text += &format!("{}  \n", step_description(&step));
  }
  text += "\n";
  text
}

fn step_description(step: &SolutionStep) -> String {
  String::from(
    format!(
      "[{}]({}) {}",
      rule_display(step.rule), rule_url(step.rule), step_details(step)
    )
  )
}

fn step_details(step: &SolutionStep) -> String {
  let cell_displays: Vec<String> = step.cells.iter().map(|cell| cell.to_string()).collect();
  let cells = cell_displays.join(", ");
  let affected_cells = step.affected_cells.iter().map(|cell| cell.to_string()).collect::<Vec<String>>().join(", ");
  let area_displays: Vec<String> = step.areas.iter().map(|area| area.to_string()).collect();
  let mut sorted_values = step.values.to_vec();
  sorted_values.sort();
  let values = sorted_values.iter().map(|val| val.to_string()).collect::<Vec<String>>().join(", ");

  match step.rule {
    Rule::HiddenSingle => {
      format!("{} in {} on cell {}", step.values[0], step.areas[0].to_string(), step.cells[0].to_string())
    },
    Rule::NakedSingle => {
      format!("{} on cell {}", step.values[0], step.cells[0].to_string())
    },
    Rule::HiddenPairs | Rule::HiddenTriples => {
      format!("of {} in {}", values, step.areas[0].to_string())
    },
    Rule::XWing => {
      format!(
        " on cells {} ({} and {}) to remove {} from {} ({} and {})",
        cells, area_displays[0], area_displays[1], values, affected_cells,
        area_displays[2], area_displays[3]
      )
    },
    Rule::XYWing => {
      let z_value = step.values[2];
      format!(
        "of {} with pivot at {} and pincers at {} and {} which removes {} from {}",
        values, cell_displays[0], cell_displays[1], cell_displays[2], z_value, affected_cells
      )
    },
    Rule::CommonPeerElimination => {
      format!(
        " to remove value {} from {} because it would eliminate it as candidate from {} (cells {})",
        values, affected_cells, step.areas[0].to_string(), cells
      )
    },
    Rule::TurbotFish => {
      format!(
        " on strong links {}-{} and {}-{}. Because {} and {} see each other, at least one 
        of {} and {} will be {}, so remove {} from cells {}",
        cell_displays[0], cell_displays[1], cell_displays[2], cell_displays[3],
        cell_displays[0], cell_displays[2], cell_displays[1], cell_displays[3],
        values, values, affected_cells
      )
    },
    Rule::LockedCandidatesPairs | Rule::LockedCandidatesTriples | Rule::NakedPairs | Rule::NakedTriples => {
      let area_message: String = if step.areas.is_empty() {
        String::default()
      } else {
        format!(" in {}", step.areas[0].to_string())
      };

      format!(
        "of {}{} to remove {} from {}",
        values, area_message, values, affected_cells,
      )
    },
    Rule::Candidates | Rule::Thermo | Rule::ThermoCandidates | Rule::KillerCandidates |
      Rule::Killer45 | Rule::Kropki | Rule::KropkiChainCandidates | Rule::TopBottomCandidates | 
      Rule::CommonPeerEliminationKropki | Rule::Swordfish => unimplemented!(),
  }
}

fn rule_display(rule: Rule) -> String {
  let s = match rule {
    Rule::NakedSingle => "Naked Single",
    Rule::HiddenSingle => "Hidden Single",
    Rule::LockedCandidatesPairs => "Locked Candidate Pair",
    Rule::NakedPairs => "Naked Pair",
    Rule::HiddenPairs => "Hidden Pair",
    Rule::CommonPeerElimination => "Common Peer Elimination",
    Rule::LockedCandidatesTriples => "Locked Candidate Triple",
    Rule::NakedTriples => "Naked Triple",
    Rule::HiddenTriples => "Hidden Triple",
    Rule::XWing => "X-Wing",
    Rule::XYWing => "XY-Wing",
    Rule::Swordfish => "Swordfish",
    Rule::TurbotFish => "Turbot Fish",
    Rule::Candidates | Rule::Thermo | Rule::ThermoCandidates | Rule::KillerCandidates | Rule::Killer45 | Rule::Kropki |
      Rule::KropkiChainCandidates | Rule::TopBottomCandidates | 
      Rule::CommonPeerEliminationKropki => unimplemented!(),
  };
  String::from(s)
}

fn rule_url(rule: Rule) -> String {
  let url = match rule {
    Rule::NakedSingle => "https://hodoku.sourceforge.net/en/tech_singles.php#n1",
    Rule::HiddenSingle => "https://hodoku.sourceforge.net/en/tech_singles.php#h1",
    Rule::LockedCandidatesPairs => "https://hodoku.sourceforge.net/en/tech_intersections.php",
    Rule::NakedPairs => "https://hodoku.sourceforge.net/en/tech_naked.php#n2",
    Rule::HiddenPairs => "https://hodoku.sourceforge.net/en/tech_hidden.php#h2",
    Rule::CommonPeerElimination => "https://lisudoku.xyz/learn#CommonPeerElimination",
    Rule::LockedCandidatesTriples => "https://hodoku.sourceforge.net/en/tech_intersections.php",
    Rule::NakedTriples => "https://hodoku.sourceforge.net/en/tech_naked.php#n3",
    Rule::HiddenTriples => "https://hodoku.sourceforge.net/en/tech_hidden.php#h3",
    Rule::XWing => "https://hodoku.sourceforge.net/en/tech_fishb.php#bf2",
    Rule::XYWing => "https://hodoku.sourceforge.net/en/tech_wings.php#xy",
    Rule::Swordfish => "https://hodoku.sourceforge.net/en/tech_fishb.php#bf3",
    Rule::TurbotFish => "https://hodoku.sourceforge.net/en/tech_sdp.php#tf",
    Rule::Candidates | Rule::Thermo | Rule::ThermoCandidates | Rule::KillerCandidates | Rule::Killer45 | Rule::Kropki |
      Rule::KropkiChainCandidates | Rule::TopBottomCandidates | 
      Rule::CommonPeerEliminationKropki => unimplemented!(),
  };
  String::from(url)
}
