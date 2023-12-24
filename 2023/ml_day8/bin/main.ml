(* Type definition for the mapping *)
type element = string
type pair = element * element
type mapping = (element, pair) Hashtbl.t

(* Function to parse a line into a tuple *)
let parse_line line =
  match String.split_on_char '=' line with
  | [key; value] ->
    let key = String.trim key in
    let values = List.map String.trim (String.split_on_char ',' (String.sub value 2 (String.length value - 3))) in
    let pair = (List.nth values 0, List.nth values 1) in
    (key, pair)
  | _ -> failwith "Invalid line format"

(* Function to read and parse the file *)
let read_file filename =
  let ic = open_in filename in
  let mapping = Hashtbl.create 10 in
  let instructions = ref "" in
  try
    instructions := input_line ic;  (* Read the first line for instructions *)
    let _ = input_line ic in         (* Read and discard the blank line *)
    while true do
      let line = input_line ic in
      let (key, pair) = parse_line line in
      Hashtbl.add mapping key pair
    done;
    (!instructions, mapping)
  with End_of_file ->
    close_in ic;
    (!instructions, mapping)

(* Function to print mappings *)
let print_mappings mapping =
  Hashtbl.iter (fun key (left, right) ->
    Printf.printf "%s = (%s, %s)\n" key left right) mapping

(* Function to check if a string ends with a given suffix *)
let ends_with str suffix =
  let len_str = String.length str in
  let len_suffix = String.length suffix in
  len_str >= len_suffix && String.sub str (len_str - len_suffix) len_suffix = suffix

(* Function to find all starting nodes *)
let find_starting_nodes mapping =
  Hashtbl.fold (fun key _ acc ->
    if ends_with key "A" then key :: acc else acc) mapping []

(* Function to calculate the greatest common divisor (GCD) *)
let rec gcd a b =
    if b = 0 then a else gcd b (a mod b)

(* Function to calculate the least common multiple (LCM) *)
let lcm a b =
  a / gcd a b * b

  (* Function to process instructions for multiple nodes and find LCM of completion *)
let process_instructions_multi mapping start_nodes instructions_str =
    let instructions = String.to_seq instructions_str |> List.of_seq in
    let completed_nodes = Hashtbl.create 10 in  (* Store completed nodes and their step counts *)
    let rec aux current_nodes remaining_instructions count =
        if count mod 10000000 = 0 then
            Printf.printf "Progress: Step %d, Current Nodes: %s\n" count (String.concat ", " current_nodes);
      Stdlib.flush Stdlib.stdout;

    (* Check each node individually and update completed nodes *)
    List.iter (fun node ->
        if ends_with node "Z" && not (Hashtbl.mem completed_nodes node) then
            Hashtbl.add completed_nodes node count
        ) current_nodes;

    if Hashtbl.length completed_nodes = List.length start_nodes then
        (* All nodes have completed, calculate LCM *)
        let start_val = 
            let counts = Hashtbl.fold (fun _ count acc -> count :: acc) completed_nodes [] in
            match counts with
        | [] -> 1  (* Default to 1 if no completed nodes *)
        | hd::_ -> hd
            in
       let lcm_value = Hashtbl.fold (fun _ count acc -> lcm count acc) completed_nodes start_val in
       (Hashtbl.fold (fun key _ acc -> key :: acc) completed_nodes [], lcm_value)
    else
        let next_instruction = match remaining_instructions with
        | [] -> List.hd instructions  (* Restart instructions from the beginning *)
        | hd::_ -> hd
       in
      let next_nodes = List.map (fun node ->
          if Hashtbl.mem mapping node then
              let (left, right) = Hashtbl.find mapping node in
              match next_instruction with
          | 'L' -> left
          | 'R' -> right
          | _ -> failwith "Invalid instruction"
        else node
              ) current_nodes in
      aux next_nodes (match remaining_instructions with
                       | [] -> List.tl instructions  (* Restart instructions from the beginning *)
                       | _::tl -> tl) (count + 1)
      in
  aux start_nodes instructions 0

  (* Main function *)
        let () =
            if Array.length Sys.argv < 2 then begin
                Printf.eprintf "Usage: %s <filename>\n" Sys.argv.(0);
    exit 1
            end;
  let filename = Sys.argv.(1) in
  let (instructions, mapping) = read_file filename in

  (* Find starting nodes *)
  let start_nodes = find_starting_nodes mapping in

  (* Process instructions starting from each node ending with 'A' *)
  let (final_nodes, steps) = process_instructions_multi mapping start_nodes instructions in
  Printf.printf "\nFinal Nodes: %s\n" (String.concat ", " final_nodes);
  Printf.printf "Total Steps: %d\n" steps

