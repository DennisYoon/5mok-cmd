use std::io::stdin;
use std::time::Duration;
use std::thread::sleep;
use clearscreen;
use colored::Colorize;

/* 현재 턴 */
enum Turn {
  Black,
  White
}

/* 에러 종류 */
enum Error {
  PlayerInput, 
  PlaceInput
}

/* 플레이어 닉네임 */
struct Player {
  black: String,
  white: String
}

fn main() {
  let foo = the_game();

  /* 게임 다시 하고프면(true) main 재귀 호출 */
  if foo {
    main();
  }
}

fn the_game() -> bool {
  /* SETTING BEFORE GAME START <START> */
  clearscreen::clear().unwrap();

  let mut black = String::new();
  let mut white = String::new();

  show_info_before_game_start();

  println!("흑돌 player의 이름을 입력하세요 >> ");
  stdin().read_line(&mut black).unwrap();
  if black.len() == 2 {
    return error(Error::PlayerInput);
  }

  println!();

  println!("백돌 player의 이름을 입력하세요 >> ");
  stdin().read_line(&mut white).unwrap();
  if white.len() == 2 {
    return error(Error::PlayerInput);
  }

  delete_trash(&mut black);
  delete_trash(&mut white);

  //플레이어 둘이 이름이 같으면 안됨!
  if black.eq(&white) {
    return error(Error::PlayerInput);
  }

  //기본 변수들
  let player = Player {black, white};
  let mut board: [i8; 225] = [0; 225];
  let mut turn = Turn::Black;
  let mut count = 1;
  let mut go_back = false;
  let mut which_stone = Vec::new();
  /* SETTING BEFORE GAME START <END> */

  /* GAME WITH A BOARD <START> */ 
  loop {
    clearscreen::clear().unwrap();

    //현재 순서, 진행 상황 출력
    match turn {
      Turn::Black => {
        println!("{:=^50}\n", format!(
          "흑돌({}) 차례({}번수)", player.black, count
        ));
      },
      Turn::White => {
        println!("{:=^50}\n", format!(
          "백돌({}) 차례({}번수)", player.white, count
        ));
      }
    };
  
    //오목판 출력
    let alphabet = String::from("abcdefghijklmno");
    println!("{}", format!("  a b c d e f g h i j k l m n o").yellow());
    for (i, ch) in alphabet.to_uppercase().chars().enumerate() {
      print!("{}", format!("{}", ch).yellow());
      for j in 0..15 {
        let print_str: &str = match board[j + 15 * i] {
          0 => "+", //board slice
          1 => "O", //black
          2 => "X", //white
          3 => "N", //banned
          _ => "&"
        };
        print!(" {}", print_str);
      }
      println!();
    }
    println!();
  
    //돌 놓을 곳 입력
    let mut place = String::new();
    println!("돌을 놓을 곳을 입력하시오 ex)'gM'은 가로 g, 세로 M자리에  >> ");
    stdin().read_line(&mut place).unwrap();
    delete_trash(&mut place);

    //입력값이 pass이면
    if place.eq("pass") {
      match turn {
        Turn::Black => {
          turn = Turn::White;
        },
        Turn::White => {
          turn = Turn::Black;
        }
      };
      continue;
    }

    if place.len() != 2 {
      error(Error::PlaceInput);
      continue;
    }
    
    //돌 놓을 곳 정수형으로 변환
    let mut num_place = [16, 16];
    for (i, v) in place.to_lowercase().chars().enumerate() {
      let num = match v {
        'a' => 0, 'b' => 1, 'c' => 2, 'd' => 3, 'e' => 4,
        'f' => 5, 'g' => 6, 'h' => 7, 'i' => 8, 'j' => 9,
        'k' => 10, 'l' => 11, 'm' => 12, 'n' => 13, 'o' => 14,
        _ => 16
      };
      if count == 1 {
        num_place[i] = 7;
      } else {
        num_place[i] = num;
      }
    }
    for i in num_place {
      if i == 16 {
        error(Error::PlaceInput);
        go_back = true;
        break;
      }
    }

    if num_place[0] + num_place[1] * 15 < 225 &&
    board[num_place[0] + num_place[1] * 15] != 0 {
      error(Error::PlaceInput);
      go_back = true;
    }

    //인풋 에러 시 -->
    if go_back {
      go_back = false;
      continue;
    }
  
    //돌 놓고, 다음 차례 진행 위한 변수 제어들
    match turn {
      Turn::Black => {
        board[num_place[0] + num_place[1] * 15] = 1;
        turn = Turn::White;
      },
      Turn::White => {
        board[num_place[0] + num_place[1] * 15] = 2;
        turn = Turn::Black;
      }
    };

    //5목이 완성되었는지 판별
    let foo = check_five(
      &mut board, &mut turn, &mut which_stone
    );
    if foo {
      return game_over(
        &mut turn, &player, &mut board, &mut which_stone ,count
      );
    }

    count = count + 1;
  }
  /* GAME WITH A BOARD <END> */
}

fn show_info_before_game_start() {
  println!("'5mok cmd 1.0'을 플레이하러 오신 당신을 환영합니다.\n");
  println!("{:=^50}\n", "게임 설명");
  println!("1) 5mok cmd는 렌주룰을 체택한다. (https://www.renju.net/rules/)");
  println!("2) 흑돌의 3*3, 4*4, 장목 등 렌주룰에 위배되는 행위나 이미 둔 칸에 돌을 두려고 하면 패배다.");
  println!("3) 알파벳 대문자 O는 흑돌, X는 백돌로, +기호는 채워지지 않은 칸으로 간주한다.");
  println!("4) pass를 입력하면 패스한다.");
  println!("5) 33, 44에 대한 사항은 아직 5mok cmd 1.0에서 구현되지 않았다.");
  println!("6) 에러 발생 시 게임은 즉시 종료됨으로 유의하시기 바란다.\n");
  println!("{:=^54}\n", "");
}

fn check_five(
  checking_board: &mut [i8; 225], now_turn: &mut Turn,
  which_stone: &mut Vec<usize>
) -> bool {
  let num_turn = match now_turn {
    Turn::Black => 2, //white
    Turn::White => 1, //black
  };

  /* CASE1 : 가로 */
  {
    let mut sequence = 0;

    for now_loc in 0..225 {
      if now_loc % 15 == 0 {
        sequence = 0;
        *which_stone = Vec::new();
      }
      if checking_board[now_loc] == num_turn {
        sequence = sequence + 1;
        which_stone.push(now_loc);
        if sequence == 5 {
          if now_loc != 224 && checking_board[now_loc + 1] == 1 && num_turn == 1 {
            *which_stone = Vec::new();
            sequence = 0;
            continue;
          }
          return true;
        }
      } else {
        sequence = 0;
        *which_stone = Vec::new();
      }
    }
  }

  /* CASE2 : 세로 */
  {
    let mut sequence = 0;

    for now_loc in 0..225 {
      let now_loc_4_vert = (now_loc % 15) * 15 + now_loc / 15;
      if now_loc_4_vert <= 14 {
        sequence = 0;
        *which_stone = Vec::new();
      }
      if checking_board[now_loc_4_vert] == num_turn {
        sequence = sequence + 1;
        which_stone.push(now_loc_4_vert);
        if sequence == 5 {
          if now_loc_4_vert != 224 && num_turn == 1 &&
          checking_board[((now_loc + 1) % 15) * 15 + (now_loc + 1) / 15] == 1 {
            *which_stone = Vec::new();
            sequence = 0;
            continue;
          }
          return true;
        }
      } else {
        sequence = 0;
        *which_stone = Vec::new();
      }
    }
  }
  /* CASE3 : 우하향 대각선 */
  {
    let mut sequence = 0;

    let mut now_loc = 1;

    loop {
      now_loc = now_loc + 14;

      if now_loc - 14 == 210 {
        sequence = 0;
        *which_stone = Vec::new();
        now_loc = 29;
      }
      else if (now_loc - 14) % 15 == 0 {
        sequence = 0;
        *which_stone = Vec::new();
        now_loc = now_loc / 15 + 1;
      }
      else if (now_loc - 14) > 210 {
        sequence = 0;
        *which_stone = Vec::new();
        now_loc = (now_loc - 210 - 14 + 2) * 15 - 1;
      } else {
        
      }
      
      let _now_loc_4_reverse = now_loc + (14 - now_loc % 15) - now_loc % 15;

      if checking_board[_now_loc_4_reverse] == num_turn {
        sequence = sequence + 1;
        which_stone.push(_now_loc_4_reverse);
        if sequence == 5 {
          if _now_loc_4_reverse != 224 
            && num_turn == 1 
            && _now_loc_4_reverse % 15 != 0 
            && _now_loc_4_reverse < 210 
            && checking_board[
              (now_loc + 14) + (14 - (now_loc + 14) % 15) - (now_loc + 14) % 15
            ] == 1 {
              *which_stone = Vec::new();
              sequence = 0;
              continue;
          }
          return true;
        }
      } else {
        sequence = 0;
        *which_stone = Vec::new();
      }

      if _now_loc_4_reverse == 224 {
        break;
      }
    }
  }

  /* CASE4 : 우상향 대각선 */
  {
    let mut sequence = 0;

    let mut now_loc = 1;
    loop {
      now_loc = now_loc + 14;

      if now_loc - 14 == 210 {
        sequence = 0;
        *which_stone = Vec::new();
        now_loc = 29;
      }
      else if (now_loc - 14) % 15 == 0 {
        sequence = 0;
        *which_stone = Vec::new();
        now_loc = now_loc / 15 + 1;
      }
      else if (now_loc - 14) > 210 {
        sequence = 0;
        *which_stone = Vec::new();
        now_loc = (now_loc - 210 - 14 + 2) * 15 - 1;
      } else {
        
      }
      
      if checking_board[now_loc] == num_turn {
        sequence = sequence + 1;
        which_stone.push(now_loc);
        if sequence == 5 {
          if now_loc != 224 
            && num_turn == 1 
            && now_loc % 15 != 0 
            && now_loc < 210 
            && checking_board[now_loc + 14] == 1 {
              *which_stone = Vec::new();
              sequence = 0;
              continue;
          }
          return true;
        }
      } else {
        sequence = 0;
        *which_stone = Vec::new();
      }

      if now_loc == 224 {
        break;
      }
    }
  }

  return false;
}

fn error(error_type: Error) -> bool {
  match error_type {
    Error::PlayerInput => {
      println!("{}", format!("An error occured.").red().bold());
      sleep(Duration::from_secs(1));
      clearscreen::clear().unwrap();

      println!("{:=^50}\n", "게임 중 에러 발생");
      println!("예러 사유 : ");
      println!("잘못된 플레이어 이름을 입력했습니다.");
      println!("최소 한 글자 이상의 길이의 플레이어 이름을 사용해야 합니다.");
      println!("또한 흑과 백돌의 플레이어 이름이 같으면 안됩니다.");
      println!("\n{:=^57}", "");
      println!("{: >57}", "by YDH");

      // 인풋값이 again이면 다시 시작, 아니면 프로그램 종료
      return again_function();
    },
    Error::PlaceInput => {
      println!("{}", format!("Your input has a PROBLEM.\n").red().bold());
      println!("돌을 놓을 곳을 입력하는 도중 잘못된 입력값이 있습니다.");
      println!("돌을 놓을 곳은 (a~o)(A~O)의 형식을 따라야 함으로 이외의 문자열을 입력했습니다.");
      println!("또한 문자열의 길이는 꼭 2여야 합니다.");
      sleep(Duration::from_secs(2));

      return true;
    }
  }
}

fn game_over(
  lose: &mut Turn, joiners: &Player, board: &mut [i8; 225],
  which_stone: &mut Vec<usize>, count: u32
) -> bool {
  println!("{}", format!("Game is over.").green().bold());
  sleep(Duration::from_secs(1));
  clearscreen::clear().unwrap();

  /* 승자, 패자 정보 처리 */
  let winner_n = match lose {
    Turn::Black => &joiners.white,
    Turn::White => &joiners.black
  };

  let loser_n = match lose {
    Turn::Black => &joiners.black,
    Turn::White => &joiners.white
  };

  let winner_c = match lose {
    Turn::Black => "백",
    Turn::White => "흑"
  };

  let loser_c = match lose {
    Turn::Black => "흑",
    Turn::White => "백"
  };

  /* 게임 결과 출력 */
  println!("{:=^50}\n", "게임 결과");
  println!("{}", format!("승자 : {}({})", winner_n , winner_c).blue());
  println!("{}", format!("패자 : {}({})\n", loser_n , loser_c).red());
  println!("결판까지 사용한 돌 : {}개", count);

  /* 최종 판 출력 */
  let alphabet = String::from("abcdefghijklmno");
    println!("{}", format!("  a b c d e f g h i j k l m n o").yellow());
    for (i, ch) in alphabet.to_uppercase().chars().enumerate() {
      print!("{}", format!("{}", ch).yellow());
      for j in 0..15 {
        let print_str: &str = match board[j + 15 * i] {
          0 => "+", // board slice
          1 => "O", // black
          2 => "X", // white
          3 => "+", // board slice
          _ => "&"
        };
        if which_stone.contains(&(j + 15 * i)) { // 초록색으로 강조
          print!(" {}", format!("{}", print_str).green().bold());
        } else {
          print!(" {}", print_str);
        }
      }
      println!();
    }
    println!();

    println!("\n{:=^54}", "");
    println!("{: >54}\n", "by YDH");

  /* 인풋값이 again이면 다시 시작, 아니면 프로그램 종료 */
  return again_function();
}

fn again_function() -> bool {
  let key_string = String::from("again");
  let mut input = String::new();

  println!("\n\n다시 시작하려면 again을, 종료하려면 again이 아닌 문자열 입력 >> ");
  stdin().read_line(&mut input).unwrap();
  delete_trash(&mut input);

  /* again 입력받으면 다시, 아니면 종료 */
  if key_string.eq(&input) {
    return true;
  } else {
    return false;
  }
}

/* stdin으로 인풋 받으면 \r\n이 포함되어있기에 두 번 pop 해주기 */
fn delete_trash(trash: &mut String) {
  trash.pop();
  trash.pop();
}