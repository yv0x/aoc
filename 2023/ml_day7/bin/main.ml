type card = { rank: char; value: int }
type hand_type = FourOfAKind | FullHouse | ThreeOfAKind | TwoPair | OnePair | HighCard
type hand = { cards: card list; bid: int; htype: hand_type; ranks: int list }

let parse_card c =
  let value = match c with
    | '2' -> 2 | '3' -> 3 | '4' -> 4 | '5' -> 5
    | '6' -> 6 | '7' -> 7 | '8' -> 8 | '9' -> 9
    | 'T' -> 10 | 'J' -> 11 | 'Q' -> 12 | 'K' -> 13 | 'A' -> 14
    | _ -> failwith "Invalid rank"
  in
  { rank = c; value = value }

let print_card card =
  Printf.printf "%c " card.rank

let parse_hand str bid =
  let cards = List.init (String.length str) (fun i -> parse_card (String.get str i)) in
  (* Placeholder for htype and ranks. Logic to set these is needed. *)
  { cards = cards; bid = bid; htype = HighCard; ranks = [] }

let count_values hand =
  List.fold_left (fun acc card ->
      let count = try List.assoc card.value acc with Not_found -> 0 in
      (card.value, count + 1) :: List.remove_assoc card.value acc
    ) [] hand.cards

let determine_hand_type hand =
  let counts = count_values hand in
  let sorted_counts = List.sort (fun (_, c1) (_, c2) -> compare c2 c1) counts in
  match sorted_counts with
  | [(v1, 4); (v2, 1)] -> { hand with htype = FourOfAKind; ranks = [v1; v2] }
  | [(v1, 3); (v2, 2)] -> { hand with htype = FullHouse; ranks = [v1; v2] }
  | [(3, _); (_, _); (_, _)] ->
      { hand with htype = ThreeOfAKind; ranks = List.map (fun c -> c.value) hand.cards }
  | [(2, _); (2, _); (_, _)] ->
      { hand with htype = TwoPair; ranks = List.map (fun c -> c.value) hand.cards }
  | [(2, _); (_, _); (_, _); (_, _)] ->
      { hand with htype = OnePair; ranks = List.map (fun c -> c.value) hand.cards }
  | _ -> { hand with htype = HighCard; ranks = List.map (fun c -> c.value) hand.cards }

let hand_type_value htype =
  match htype with
  | FourOfAKind -> 7
  | FullHouse -> 6
  | ThreeOfAKind -> 5
  | TwoPair -> 4
  | OnePair -> 3
  | HighCard -> 2

let rec compare_card_lists l1 l2 =
  match (l1, l2) with
  | [], [] -> 0
  | h1::t1, h2::t2 ->
      let c = compare h1 h2 in
      if c = 0 then compare_card_lists t1 t2 else c
  | _ -> failwith "Invalid card list comparison"

let compare_hands h1 h2 =
  match compare (hand_type_value h1.htype) (hand_type_value h2.htype) with
  | 0 -> compare_card_lists h1.ranks h2.ranks
  | c -> c

let calculate_winnings hands =
    let sorted_hands = List.sort compare_hands hands in
    Printf.printf "Sorted Hands:\n";
    List.iter (fun h ->
        List.iter print_card h.cards;
        Printf.printf ": Bid %d\n" h.bid) sorted_hands;
    List.fold_left (fun acc (hand, rank) -> acc + hand.bid * rank) 0 (List.mapi (fun i h -> (h, i + 1)) sorted_hands)

let () =
  let hands = [
    parse_hand "32T3K" 765;
    parse_hand "T55J5" 684;
    parse_hand "KK677" 28;
    parse_hand "KTJJT" 220;
    parse_hand "QQQJA" 483;
    (* Add more hands as needed *)
  ] in
  let total_winnings = calculate_winnings hands in
  Printf.printf "Total winnings: %d\n" total_winnings

