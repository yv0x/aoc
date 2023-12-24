let read_grid filename =
  let in_channel = open_in filename in
  let rec read_lines channel lines =
    try
      let line = input_line channel in
      read_lines channel (line :: lines)
    with End_of_file -> List.rev lines
  in
  let lines = read_lines in_channel [] in
  close_in in_channel;
  Array.of_list (List.map (fun line -> Array.init (String.length line) (String.get line)) lines)

let print_grid grid =
  Array.iter (fun row ->
    Array.iter (Printf.printf "%c") row;
    print_newline ()
  ) grid

let () =
  let grid = read_grid "example1.txt" in
  print_grid grid; (* Print the grid for debugging *)

  let height = Array.length grid in
  let width = Array.length grid.(0) in

  let is_valid x y = x >= 0 && y >= 0 && x < width && y < height in

  let rec find_start x y =
    if y >= height then None
    else if x >= width then find_start 0 (y + 1)
    else if grid.(y).(x) = 'S' then Some (x, y)
    else find_start (x + 1) y
  in

  let directions = [|(0, -1); (1, 0); (0, 1); (-1, 0)|] in

  let connects (dx, dy) tile =
    match (dx, dy), tile with
    | (0, -1), ('|' | 'S' | '7' | 'J') -> true
    | (1, 0), ('-' | 'S' | 'L' | 'F') -> true
    | (0, 1), ('|' | 'S' | 'L' | 'F') -> true
    | (-1, 0), ('-' | 'S' | '7' | 'J') -> true
    | _, _ -> false
  in

  let find_first_connecting_direction directions tile =
    let rec aux i =
      if i >= Array.length directions then None
      else if connects directions.(i) tile then Some directions.(i)
      else aux (i + 1)
    in
    aux 0
  in

  let rec traverse x y dir dist =
    let next_x = x + fst dir in
    let next_y = y + snd dir in
    if not (is_valid next_x next_y) then dist
    else
      let next_tile = grid.(next_y).(next_x) in
      if next_tile = '.' then dist
      else
        match find_first_connecting_direction directions next_tile with
        | Some new_dir ->
            if (fst new_dir, snd new_dir) = (- fst dir, - snd dir) then dist
            else traverse next_x next_y new_dir (dist + 1)
        | None -> dist
  in

   match find_start 0 0 with
  | Some (sx, sy) ->
    Printf.printf "Starting tile found at: (%d, %d)\n" sx sy;  (* Print the starting position *)
    let initial_dir = 
      match find_first_connecting_direction directions grid.(sy).(sx) with
      | Some dir -> dir
      | None -> failwith "Invalid starting position"
    in
    let farthest_dist = traverse sx sy initial_dir 0 in
    Printf.printf "Farthest distance from start: %d\n" farthest_dist
  | None -> Printf.printf "Starting point not found\n"
