(* Define types for your data structures *)
type map = (int * int * int) list

(* Function to safely convert string to integer with error handling *)
let safe_int_of_string s =
  try Some (int_of_string s)
  with Failure _ -> None

(* Function to parse a line into a list of integers *)
let parse_line_to_list line =
  List.filter_map safe_int_of_string (String.split_on_char ' ' line)

(* Function to parse a line into a tuple of three integers *)
let parse_line_to_tuple line =
  match parse_line_to_list line with
  | [a; b; c] -> Some (a, b, c)
  | _ -> None

(* Function to read and process the file *)
let process_file filename =
  let in_channel = open_in filename in
  let rec read_lines times distances =
    try
      let line = input_line in_channel in
      if String.trim line = "" then
        read_lines times distances 
      else if String.length line >= 6 && String.sub line 0 6 = "Time:" then
        read_lines "time" (List.append times (parse_line_to_list (String.sub line 5 (String.length line - 5)))) 
      else if String.contains line ':' then
        let new_section = String.sub line 0 (String.index line ':') in
        print_endline ("Switching to section: " ^ new_section);
        read_lines new_section seeds seed_to_soil_map
      else
        match current_section, parse_line_to_tuple line with
        | "seed-to-soil map", Some numbers -> 
            print_endline ("Parsing seed-to-soil line: " ^ line);
            read_lines current_section seeds (numbers :: seed_to_soil_map)
        | _ -> read_lines current_section seeds seed_to_soil_map
    with
    | End_of_file -> close_in in_channel; (seeds, seed_to_soil_map)
  in
  read_lines "" [] []

(* Main function *)
let () =
  let (times, distances) = process_file "example.txt" in
  (* Print the seeds and maps *)
  Printf.printf "Time: ";
  List.iter (Printf.printf "%d ") times;
  Printf.printf "\nDistance:\n";
  List.iter (Printf.printf "%d ") distances;


