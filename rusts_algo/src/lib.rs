pub mod add_two_int_without_carry;
pub mod all_longest_strings;
pub mod almost_increasing_seq;
pub mod alhpabet_subseq;
pub mod appear_twice;
pub mod applebox;
pub mod arithmatic_progression;
pub mod array_is_smooth;
pub mod array_product_sum;
pub mod average;
pub mod binarysearch;
pub mod bincount;
pub mod bintodec;
pub mod bt_from_inorder_preorder;
pub mod bubble_sort;
pub mod build_closure_tag;
pub mod build_heap;
pub mod build_ip_addr;
pub mod build_palindrome;
pub mod caesar_crypt;
pub mod camelcase;
pub mod can_increase_with_roundness;
pub mod can_make_sum_a_with_k_num;
pub mod capitalize;
pub mod celcius_to_fahrenheit;
pub mod code_breaker;
pub mod century;
pub mod char_count;
pub mod chars_appearing_twice;
pub mod chars_perm_list;
pub mod chars_to_sorted_digits;
pub mod christmas_tree;
pub mod clamp_value;
pub mod classmark;
pub mod clean_kth_bit;
pub mod combination;
pub mod compare_and_swap;
pub mod construct_min_height_bst;
pub mod construct_square;
pub mod contrast_check;
pub mod cool_num_pair;
pub mod cross_entropy;
pub mod cyclic_chars;
pub mod cumsum_except_last;
pub mod decode_reverse_poland;
pub mod dectobin;
pub mod deg_to_rad;
pub mod diagonal;
pub mod different_squares;
pub mod different_value_pairs;
pub mod dfs;
pub mod dns_water_torture;
pub mod document_build;
pub mod euclidean;
pub mod eval_tictactoe;
pub mod even_num_sum;
pub mod factorial;
pub mod fahrenheit_to_celcius;
pub mod fibonacci;
pub mod file_naming;
pub mod find_closest_value;
pub mod find_max_val_thread;
pub mod find_successor;
pub mod first_dupe_value;
pub mod fizzbuzz;
pub mod fsm_mealy;
pub mod fsm_moor;
pub mod geometric_progression;
pub mod geometrical_mean;
pub mod least_greatest;
pub mod group_anagrams;
pub mod group_by_class_range;
pub mod group_by_key;
pub mod guard_technique;
pub mod hanoi_tower;
pub mod harmonic_mean;
pub mod helon_formula;
pub mod human_and_cat_legs;
pub mod image_stride;
pub mod inorder_traversal;
pub mod invert_btree;
pub mod insertion_sort;
pub mod ipv4;
pub mod is_bst;
pub mod is_mac_addr;
pub mod is_palindrome;
pub mod is_two_array_similar;
pub mod island_size;
pub mod jump_height_in_frames;
pub mod kadane;
pub mod knapsack;
pub mod kth_element_in_tree;
pub mod l_r_value_sum_combo;
pub mod lcs;
pub mod largest_adjacent_product;
pub mod largest_number;
pub mod least_data_eviction;
pub mod least_factorial;
pub mod least_lsd;
pub mod lagrange_interpolation_polynominal;
pub mod length_linked_list;
pub mod levenshtein_distance;
pub mod linked_list;
pub mod loc_by_line_angle;
pub mod longest_pelindromic_substring;
pub mod longest_substring_without_dupe;
pub mod lower_upper_hinge;
pub mod l0_norm;
pub mod l1_norm;
pub mod max_path_sum;
pub mod max_sibling_product;
pub mod max_subset_sum;
pub mod max_with_lessdigit;
pub mod maxnum_by_del_one_digit;
pub mod mean;
pub mod merge_2_lists;
pub mod merge_sorted_linkedlist;
pub mod min_breakdown_sum;
pub mod min_passes_matrix_update;
pub mod min_reward;
pub mod minesweeper;
pub mod minimum_waiting_game;
pub mod minmax_stack;
pub mod mode;
pub mod monotonic_array;
pub mod most_frequent_digit_sum;
pub mod n_steps;
pub mod nega_posi_inversion;
pub mod next_greater_element;
pub mod node_distance_k;
pub mod num_of_clans;
pub mod num_of_paths;
pub mod number_grouping;
pub mod obtain_increasing_seq;
pub mod octal_permission;
pub mod octal_to_dec;
pub mod oppsite_pos_in_circle;
pub mod overlapping_intervals;
pub mod partial_sum_count;
pub mod pascal_triangle;
pub mod peak_in_array;
pub mod permutation;
pub mod permutation_matrix;
pub mod phone_mnemonic;
pub mod prefix_sums;
pub mod postorder_traversal;
pub mod powerset;
pub mod preorder_traversal;
pub mod product_array_sort;
pub mod pseudo_rand;
pub mod quarter;
pub mod quick_sort;
pub mod rad_to_deg;
pub mod radix_sort;
pub mod random_perm;
pub mod ranking;
pub mod reconstruct_bst_from_pre;
pub mod remove_island;
pub mod remove_nth_from_end;
pub mod repeat_product;
pub mod replace_mid_value;
pub mod request_per_sec;
pub mod reverse_in_parenthiesis;
pub mod reverse_stack;
pub mod reverse_poland_calc;
pub mod reverse_words;
pub mod rgb_to_bgr;
pub mod rgb_to_yuv;
pub mod rightmost_diffbit;
pub mod rightmost_samebit;
pub mod ring_buffer;
pub mod runlength;
pub mod same_bst;
pub mod selection_sort;
pub mod shapearea;
pub mod shared;
pub mod shorten_path;
pub mod sturge_formula;
pub mod sigma_k;
pub mod single_cycle_check;
pub mod single_stroke;
pub mod singly_linked_list_copy;
pub mod singly_linked_list_reverse;
pub mod smallest_difference;
pub mod smallest_positive_product;
pub mod smallest_sum;
pub mod sprite_index;
pub mod softmax;
pub mod softsign;
pub mod sort_by_height;
pub mod sort_stack;
pub mod sorted_matrix_search;
pub mod stack_copy_only_pop;
pub mod stdev;
pub mod storage_projection_for_sdhd;
pub mod strange_bank;
pub mod string_construction;
pub mod string_pattern;
pub mod subarray_sort;
pub mod suffix_trie;
pub mod sudoku;
pub mod sum_of_arithmatic_progression;
pub mod sum_of_consecutive_integers;
pub mod sum_of_integers;
pub mod sum_of_squared_deviation;
pub mod sunset_view;
pub mod swap_sibling;
pub mod tandem_bike;
pub mod tandem_repeat;
pub mod task_assignment;
pub mod three_number_sort;
pub mod toggle_bit;
pub mod topological_sort;
pub mod total_depth_sum_of_tree;
pub mod total_sum_with_n_num;
pub mod three_sum;
pub mod turn_commands;
pub mod two_sum;
pub mod unconstructiable_number;
pub mod valid_parentheses;
pub mod valid_starting_city;
pub mod validate_three_nodes;
pub mod variance;
pub mod waterarea;
pub mod xor_shift;
pub mod yuv_to_rgb;
pub mod z_score;
pub mod zigzag_traversal;

use clap::Parser;

pub mod runner {

    use super::*;
    use add_two_int_without_carry::add_two_without_carry;
    use all_longest_strings;
    use almost_increasing_seq::almost_increasing_seq;
    use alhpabet_subseq;
    use appear_twice::appear_twice;
    use applebox::applebox;
    use arithmatic_progression;
    use array_is_smooth;
    use array_product_sum::array_product_sum;
    use average;
    use binarysearch::binary_search;
    use bincount;
    use bintodec::bin_to_dec;
    use bt_from_inorder_preorder;
    use bubble_sort;
    use build_closure_tag;
    use build_heap;
    use build_ip_addr;
    use build_palindrome;
    use caesar_crypt::caesar_crypt;
    use camelcase::camelcase;
    use can_increase_with_roundness;
    use can_make_sum_a_with_k_num;
    use capitalize;
    use century::century;
    use char_count;
    use chars_appearing_twice;
    use chars_to_sorted_digits;
    use christmas_tree::christmas_tree;
    use clamp_value;
    use classmark;
    use clean_kth_bit::clean_kth_bit;
    use code_breaker;
    use combination;
    use compare_and_swap;
    use construct_min_height_bst;
    use construct_square;
    use contrast_check;
    use cool_num_pair::cool_num_pair;
    use cross_entropy;
    use cumsum_except_last;
    use cyclic_chars::cyclic_chars;
    use decode_reverse_poland::decode_reverse_poland;
    use dectobin::dectobin;
    use deg_to_rad;
    use dfs;
    use diagonal::diagonal;
    use different_squares;
    use different_value_pairs;
    use dns_water_torture;
    use document_build::document_build;
    use euclidean::euclidean;
    use eval_tictactoe;
    use even_num_sum::even_num_sum;
    use factorial;
    use fahrenheit_to_celcius;
    use fibonacci;
    use file_naming;
    use find_closest_value;
    use find_max_val_thread;
    use find_successor;
    use first_dupe_value;
    use fizzbuzz::fizzbuzz;
    use fsm_mealy;
    use geometric_progression;
    use geometrical_mean;
    use group_anagrams;
    use group_by_class_range;
    use group_by_key;
    use guard_technique;
    use hanoi_tower;
    use harmonic_mean;
    use helon_formula::helon_formula;
    use human_and_cat_legs;
    use jump_height_in_frames;
    use image_stride;
    use inorder_traversal;
    use insertion_sort;
    use invert_btree;
    use ipv4;
    use is_bst;
    use is_mac_addr::is_mac_addr;
    use is_palindrome::is_palindrome;
    use is_two_array_similar;
    use island_size;
    use kadane::kadane;
    use knapsack;
    use kth_element_in_tree;
    use l_r_value_sum_combo;
    use lagrange_interpolation_polynominal;
    use largest_adjacent_product;
    use largest_number;
    use lcs::lcs;
    use least_data_eviction::least_data_eviction;
    use least_factorial::least_factorial;
    use least_greatest;
    use least_lsd;
    use length_linked_list;
    use levenshtein_distance;
    use linked_list;
    use loc_by_line_angle;
    use longest_pelindromic_substring;
    use longest_substring_without_dupe;
    use lower_upper_hinge;
    use l0_norm;
    use l1_norm;
    use max_path_sum;
    use max_sibling_product::max_sibling_product;
    use max_with_lessdigit::max_with_lessdigit;
    use maxnum_by_del_one_digit;
    use mean;
    use merge_2_lists;
    use merge_sorted_linkedlist;
    use min_breakdown_sum::min_breakdown_sum;
    use min_passes_matrix_update;
    use min_reward::min_reward;
    use minesweeper;
    use minimum_waiting_game;
    use minmax_stack;
    use mode;
    use monotonic_array;
    use most_frequent_digit_sum;
    use n_steps;
    use nega_posi_inversion;
    use next_greater_element;
    use node_distance_k;
    use num_of_clans;
    use num_of_paths;
    use number_grouping;
    use obtain_increasing_seq;
    use octal_permission;
    use octal_to_dec;
    use oppsite_pos_in_circle;
    use overlapping_intervals;
    use pascal_triangle;
    use partial_sum_count;
    use peak_in_array;
    use permutation;
    use permutation_matrix;
    use phone_mnemonic;
    use postorder_traversal;
    use powerset;
    use prefix_sums;
    use preorder_traversal::preorder_traversal;
    use product_array_sort::product_array_sort;
    use pseudo_rand;
    use quarter;
    use quick_sort;
    use rad_to_deg;
    use radix_sort::radix_sort;
    use random_perm;
    use ranking;
    use reconstruct_bst_from_pre;
    use remove_island;
    use remove_nth_from_end;
    use repeat_product::repeat_product;
    use replace_mid_value;
    use request_per_sec::request_per_sec;
    use reverse_in_parenthiesis;
    use reverse_poland_calc::reverse_poland_calc;
    use reverse_stack;
    use reverse_words;
    use rgb_to_bgr;
    use rgb_to_yuv;
    use rightmost_diffbit::rightmost_diffbit;
    use rightmost_samebit::rightmost_samebit;
    use ring_buffer;
    use runlength::runlength;
    use same_bst;
    use shapearea;
    use selection_sort::selection_sort;
    use shorten_path;
    use sigma_k::sigma_k;
    use single_cycle_check;
    use single_stroke;
    use singly_linked_list_copy;
    use singly_linked_list_reverse;
    use smallest_difference;
    use smallest_positive_product::smallest_positive_product;
    use smallest_sum;
    use sprite_index;
    use softmax;
    use softsign;
    use sort_by_height;
    use sort_stack;
    use sorted_matrix_search;
    use suffix_trie;
    use stack_copy_only_pop;
    use stdev;
    use storage_projection_for_sdhd;
    use strange_bank;
    use string_construction::string_construction;
    use string_pattern::string_pattern;
    use sturge_formula;
    use subarray_sort;
    use sudoku;
    use sum_of_arithmatic_progression;
    use sum_of_consecutive_integers;
    use sum_of_integers;
    use sum_of_squared_deviation;
    use sunset_view;
    use swap_sibling::swap_sibling;
    use tandem_bike;
    use tandem_repeat::tandem_repeat;
    use task_assignment;
    use three_number_sort;
    use three_sum;
    use toggle_bit::toggle_bit;
    use topological_sort;
    use total_depth_sum_of_tree;
    use total_sum_with_n_num;
    use turn_commands;
    use two_sum::two_sum;
    use unconstructiable_number;
    use valid_parentheses;
    use valid_starting_city;
    use validate_three_nodes;
    use variance;
    use waterarea::waterarea;
    use xor_shift;
    use yuv_to_rgb;
    use z_score;
    use zigzag_traversal;

    pub fn exec(algo: Algo) {
        match algo {
            Algo::AddTwoIntWithoutCarry => {
                add_two_without_carry::run();
            }
            Algo::AllLongestStrings => {
                all_longest_strings::run();
            }
            Algo::AlmostIncreasingSeq => {
                almost_increasing_seq::run();
            }
            Algo::AlphabetSubseq => {
                alhpabet_subseq::run();
            }
            Algo::AppearTwice => {
                appear_twice::run();
            }
            Algo::Applebox => {
                applebox::run();
            }
            Algo::ArithmaticProgression => {
                arithmatic_progression::run();
            }
            Algo::ArrayIsSmooth => {
                array_is_smooth::run();
            }
            Algo::ArrayProductSum => {
                array_product_sum::run();
            }
            Algo::Average => {
                average::run();
            }
            Algo::BinarySearch => {
                binary_search::run();
            }
            Algo::Bincount => {
                bincount::run();
            }
            Algo::BinToDec => {
                bin_to_dec::run();
            }
            Algo::BubbleSort => {
                bubble_sort::run();
            }
            Algo::BuildBTreeFromInorderPreorder => {
                bt_from_inorder_preorder::run();
            }
            Algo::BuildClosureTag => {
                build_closure_tag::run();
            }
            Algo::BuildHeap => {
                build_heap::run();
            }
            Algo::BuildIPAddr => {
                build_ip_addr::run();
            }
            Algo::BuildPalindrome => {
                build_palindrome::run();
            }
            Algo::CaesarCrypt => {
                caesar_crypt::run();
            }
            Algo::CamelCase => {
                camelcase::run();
            }
            Algo::CanIncreaseWithRoundness => {
                can_increase_with_roundness::run();
            }
            Algo::CanMakeSumAWithKNum => {
                can_make_sum_a_with_k_num::run();
            }
            Algo::Capitalize => {
                capitalize::run();
            }
            Algo::CelciusToFahrenheit => {
                celcius_to_fahrenheit::run();
            }
            Algo::Century => {
                century::run();
            }
            Algo::CharCount => {
                char_count::run();
            }
            Algo::CharsAppearingTwice => {
                chars_appearing_twice::run();
            }
            Algo::CharsPermList => {
                chars_perm_list::run();
            }
            Algo::CharsToSortedDigits => {
                chars_to_sorted_digits::run();
            }
            Algo::ChristmasTree => {
                christmas_tree::run();
            }
            Algo::ClampValue => {
                clamp_value::run();
            }
            Algo::Classmark => {
                classmark::run();
            }
            Algo::CleanKthBit => {
                clean_kth_bit::run();
            }
            Algo::CodeBreaker => {
                code_breaker::run();
            }
            Algo::Combination => {
                combination::run();
            }
            Algo::CompareAndSwap => {
                compare_and_swap::run();
            }
            Algo::ConstructMinHeightBST => {
                construct_min_height_bst::run();
            }
            Algo::ConstructSquare => {
                construct_square::run();
            }
            Algo::ContrastCheck => {
                contrast_check::run();
            }
            Algo::CoolNumPair => {
                cool_num_pair::run();
            }
            Algo::CrossEntropy => {
                cross_entropy::run();
            }
            Algo::CumsumExceptLast => {
                cumsum_except_last::run();
            }
            Algo::CyclicChars => {
                cyclic_chars::run();
            }
            Algo::DecodeReversePoland => {
                decode_reverse_poland::run();
            }
            Algo::DecToBin => {
                dectobin::run();
            }
            Algo::DegToRad => {
                deg_to_rad::run();
            }
            Algo::DFS => {
                dfs::run();
            }
            Algo::Diagonal => {
                diagonal::run();
            }
            Algo::DifferentSquares => {
                different_squares::run();
            }
            Algo::DifferentValuePairs => {
                different_value_pairs::run();
            }
            Algo::DNSWaterTorture => {
                dns_water_torture::run();
            }
            Algo::DocumentBuild => {
                document_build::run();
            }
            Algo::EvalTicTacToe => {
                eval_tictactoe::run();
            }
            Algo::EvenNumSum => {
                even_num_sum::run();
            }
            Algo::Euclidean => {
                euclidean::run();
            }
            Algo::Factorial => {
                factorial::run();
            }
            Algo::FahrenheitToCelcius => {
                fahrenheit_to_celcius::run();
            }
            Algo::Fibonacci => {
                fibonacci::run();
            }
            Algo::FileNaming => {
                file_naming::run();
            }
            Algo::FindClosestValue => {
                find_closest_value::run();
            }
            Algo::FindMaxValThread => {
                find_max_val_thread::run();
            }
            Algo::FindSuccessor => {
                find_successor::run();
            }
            Algo::FirstDupeValue => {
                first_dupe_value::run();
            }
            Algo::FizzBuzz => {
                fizzbuzz::run();
            }
            Algo::FSMMealy => {
                fsm_mealy::run();
            }
            Algo::FSMMoor => {
                fsm_moor::run();
            }
            Algo::GeometricProgression => {
                geometric_progression::run();
            }
            Algo::GeometricalMean => {
                geometrical_mean::run();
            }
            Algo::GroupAnagrams => {
                group_anagrams::run();
            }
            Algo::GroupByClassRange => {
                group_by_class_range::run();
            }
            Algo::GroupByKey => {
                group_by_key::run();
            }
            Algo::GuardTechnique => {
                guard_technique::run();
            }
            Algo::HanoiTower => {
                hanoi_tower::run();
            }
            Algo::HarmonicMean => {
                harmonic_mean::run();
            }
            Algo::HelonFormula => {
                helon_formula::run();
            }
            Algo::HumanAndCatlegs => {
                human_and_cat_legs::run();
            }
            Algo::ImageStride => {
                image_stride::run();
            }
            Algo::InOrderTraversal => {
                inorder_traversal::run();
            }
            Algo::InsertionSort => {
                insertion_sort::run();
            }
            Algo::InvertBTree => {
                invert_btree::run();
            }
            Algo::IPv4 => {
                ipv4::run();
            }
            Algo::IsBST => {
                is_bst::run();
            }
            Algo::IslandSize => {
                island_size::run();
            }
            Algo::IsPalindrome => {
                is_palindrome::run();
            }
            Algo::IsMacAddr => {
                is_mac_addr::run();
            }
            Algo::IsTwoArraySimilar => {
                is_two_array_similar::run();
            }
            Algo::JumpHeightInFrames => {
                jump_height_in_frames::run();
            }
            Algo::Kadane => {
                kadane::run();
            }
            Algo::Knapsack => {
                knapsack::run();
            }
            Algo::KthElementInTree => {
                kth_element_in_tree::run();
            }
            Algo::LagrangeInterpolationPolynominal => {
                lagrange_interpolation_polynominal::run();
            }
            Algo::LargestAdjacentProduct => {
                largest_adjacent_product::run();
            }
            Algo::LargestNumber => {
                largest_number::run();
            }
            Algo::LCS => {
                lcs::run();
            }
            Algo::LengthOfLinkedList => {
                length_linked_list::run();
            }
            Algo::LinkedList => {
                linked_list::run();
            }
            Algo::LeastDataEviction => {
                least_data_eviction::run();
            }
            Algo::LeastFactorial => {
                least_factorial::run();
            }
            Algo::LeastGreatest => {
                least_greatest::run();
            }
            Algo::LeastLSD => {
                least_lsd::run();
            }
            Algo::LevenShteinDistance => {
                levenshtein_distance::run();
            }
            Algo::LocByLineAngle => {
                loc_by_line_angle::run();
            }
            Algo::LongestPelindromicSubstring => {
                longest_pelindromic_substring::run();
            }
            Algo::LongestSubstringWithoutDupe => {
                longest_substring_without_dupe::run();
            }
            Algo::LowerUpperHinge => {
                lower_upper_hinge::run();
            }
            Algo::L0Norm => {
                l0_norm::run();
            }
            Algo::L1Norm => {
                l1_norm::run();
            }
            Algo::LRValueSumCombo => {
                l_r_value_sum_combo::run();
            }
            Algo::MaxNumByDelOneDigit => {
                maxnum_by_del_one_digit::run();
            }
            Algo::MaxPathSum => {
                max_path_sum::run();
            }
            Algo::MaxSiblingProduct => {
                max_sibling_product::run();
            }
            Algo::MaxSubSetSum => {
                max_subset_sum::run();
            }
            Algo::MaxWithLessDigit => {
                max_with_lessdigit::run();
            }
            Algo::Mean => {
                mean::run();
            }
            Algo::Merge2Lists => {
                merge_2_lists::run();
            }
            Algo::MergeSortedLinkedList => {
                merge_sorted_linkedlist::run();
            }
            Algo::MinBreakdownSum => {
                min_breakdown_sum::run();
            }
            Algo::Minesweeper => {
                minesweeper::run();
            }
            Algo::MinimumWaitingGame => {
                minimum_waiting_game::run();
            }
            Algo::MinMaxStack => {
                minmax_stack::run();
            }
            Algo::MinPassesMatrixUpdate => {
                min_passes_matrix_update::run();
            }
            Algo::MinReward => {
                min_reward::run();
            }
            Algo::Mode => {
                mode::run();
            }
            Algo::MonotonicArray => {
                monotonic_array::run();
            }
            Algo::MostFrequentDigitSum => {
                most_frequent_digit_sum::run();
            }
            Algo::NextGreaterElement => {
                next_greater_element::run();
            }
            Algo::NodeDistanceK => {
                node_distance_k::run();
            }
            Algo::NSteps => {
                n_steps::run();
            }
            Algo::NegaPosiInversion => {
                nega_posi_inversion::run();
            }
            Algo::NumOfClans => {
                num_of_clans::run();
            }
            Algo::NumOfPaths => {
                num_of_paths::run();
            }
            Algo::NumberGrouping => {
                number_grouping::run();
            }
            Algo::ObtainIncreasingSeq => {
                obtain_increasing_seq::run();
            }
            Algo::OctalPermission => {
                octal_permission::run();
            }
            Algo::OctalToDec => {
                octal_to_dec::run();
            }
            Algo::OppositePosInCircle => {
                oppsite_pos_in_circle::run();
            }
            Algo::OverlappingIntervals => {
                overlapping_intervals::run();
            }
            Algo::PartialSumCount => {
                partial_sum_count::run();
            }
            Algo::PascalTriangle => {
                pascal_triangle::run();
            }
            Algo::PeakInArray => {
                peak_in_array::run();
            }
            Algo::Permutation => {
                permutation::run();
            }
            Algo::PermutationMatrix => {
                permutation_matrix::run();
            }
            Algo::PhoneMnemonic => {
                phone_mnemonic::run();
            }
            Algo::PostOrderTraversal => {
                postorder_traversal::run();
            }
            Algo::PowerSet => {
                powerset::run();
            }
            Algo::PrefixSums => {
                prefix_sums::run();
            }
            Algo::PreOrderTraversal => {
                preorder_traversal::run();
            }
            Algo::ProductArraySort => {
                product_array_sort::run();
            }
            Algo::PseudoRand => {
                pseudo_rand::run();
            }
            Algo::Quarter => {
                quarter::run();
            }
            Algo::QuickSort => {
                quick_sort::run();
            }
            Algo::RadToDeg => {
                rad_to_deg::run();
            }
            Algo::RandomPerm => {
                random_perm::run();
            }
            Algo::Ranking => {
                ranking::run();
            }
            Algo::ReconstructBSTFromPreorder => {
                reconstruct_bst_from_pre::run();
            }
            Algo::RadixSort => {
                radix_sort::run();
            }
            Algo::RemoveIsland => {
                remove_island::run();
            }
            Algo::RemoveNthFromEnd => {
                remove_nth_from_end::run();
            }
            Algo::RepeatProduct => {
                repeat_product::run();
            }
            Algo::ReplaceMidValue => {
                replace_mid_value::run();
            }
            Algo::RequestPerSec => {
                request_per_sec::run();
            }
            Algo::ReverseInParenthiesis => {
                reverse_in_parenthiesis::run();
            }
            Algo::ReversePoland => {
                reverse_poland_calc::run();
            }
            Algo::ReverseStack => {
                reverse_stack::run();
            }
            Algo::ReverseWords => {
                reverse_words::run();
            }
            Algo::RgbToBgr => {
                rgb_to_bgr::run();
            }
            Algo::RgbToYuv => {
                rgb_to_yuv::run();
            }
            Algo::RightMostDiffBit => {
                rightmost_diffbit::run();
            }
            Algo::RightMostSameBit => {
                rightmost_samebit::run();
            }
            Algo::RingBuffer => {
                ring_buffer::run();
            }
            Algo::RunLength => {
                runlength::run();
            }
            Algo::SameBST => {
                same_bst::run();
            }
            Algo::ShapeArea => {
                shapearea::run();
            }
            Algo::SelectionSort => {
                selection_sort::run();
            }
            Algo::ShortenPath => {
                shorten_path::run();
            }
            Algo::SigmaK => {
                sigma_k::run();
            }
            Algo::SingleCycleCheck => {
                single_cycle_check::run();
            }
            Algo::SingleStroke => {
                single_stroke::run();
            }
            Algo::SinglyLinkedListCopy => {
                singly_linked_list_copy::run();
            }
            Algo::SinglyLinkedListReverse => {
                singly_linked_list_reverse::run();
            }
            Algo::SmallestDifference => {
                smallest_difference::run();
            }
            Algo::SmallestPositiveProduct => {
                smallest_positive_product::run();
            }
            Algo::SmallestSum => {
                smallest_sum::run();
            }
            Algo::SpriteIndex => {
                sprite_index::run();
            }
            Algo::Softmax => {
                softmax::run();
            }
            Algo::Softsign => {
                softsign::run();
            }
            Algo::SortByHeight => {
                sort_by_height::run();
            }
            Algo::SortStack => {
                sort_stack::run();
            }
            Algo::SortedMatrixSearch => {
                sorted_matrix_search::run();
            }
            Algo::StackCopyOnlyPop => {
                stack_copy_only_pop::run();
            }
            Algo::Stdev => {
                stdev::run();
            }
            Algo::StorageProjectionForSDHD => {
                storage_projection_for_sdhd::run();
            }
            Algo::StrangeBank => {
                strange_bank::run();
            }
            Algo::StringConstruction => {
                string_construction::run();
            }
            Algo::StringPattern => {
                string_pattern::run();
            }
            Algo::SturgeFormula => {
                sturge_formula::run();
            }
            Algo::SubArraySort => {
                subarray_sort::run();
            }
            Algo::Sudoku => {
                sudoku::run();
            }
            Algo::SuffixTrue => {
                suffix_trie::run();
            }
            Algo::SumOfArithmaticProgression => {
                sum_of_arithmatic_progression::run();
            }
            Algo::SumOfConsective => {
                sum_of_consecutive_integers::run();
            }
            Algo::SumOfIntegers => {
                sum_of_integers::run();
            }
            Algo::SumOfSquaredDeviations => {
                sum_of_squared_deviation::run();
            }
            Algo::SunsetView => {
                sunset_view::run();
            }
            Algo::SwapSibling => {
                swap_sibling::run();
            }
            Algo::TandemBike => {
                tandem_bike::run();
            }
            Algo::TandemRepeat => tandem_repeat::run(),
            Algo::TaskAssignment => task_assignment::run(),
            Algo::ThreeNumberSort => {
                three_number_sort::run();
            }
            Algo::ToggleBit => {
                toggle_bit::run();
            }
            Algo::ThreeSum => {
                three_sum::run();
            }
            Algo::TopologicalSort => {
                topological_sort::run();
            }
            Algo::TotalDepthSumOfTree => {
                total_depth_sum_of_tree::run();
            }
            Algo::TotalSumWithNNums => {
                total_sum_with_n_num::run();
            }
            Algo::TurnCommands => {
                turn_commands::run();
            }
            Algo::TwoSum => {
                two_sum::run();
            }
            Algo::UnconstructiableNumber => {
                unconstructiable_number::run();
            }
            Algo::ValidateThreeNodes => {
                validate_three_nodes::run();
            }
            Algo::ValidParentheses => {
                valid_parentheses::run();
            }
            Algo::ValidStartingCity => {
                valid_starting_city::run();
            }
            Algo::Variance => {
                variance::run();
            }
            Algo::WaterArea => {
                waterarea::run();
            }
            Algo::XOrShift => {
                xor_shift::run();
            }
            Algo::YuvToRgb => {
                yuv_to_rgb::run();
            }
            Algo::zigzagTraversal => {
                zigzag_traversal::run();
            }
            Algo::ZScore => {
                z_score::run();
            }
        }
    }
}

#[cfg(test)]
mod test_runner {
    use crate::add_two_int_without_carry::add_two_without_carry;
    use crate::all_longest_strings;
    use crate::almost_increasing_seq::almost_increasing_seq;
    use crate::alhpabet_subseq;
    use crate::appear_twice::appear_twice;
    use crate::applebox::applebox;
    use crate::arithmatic_progression;
    use crate::array_is_smooth;
    use crate::array_product_sum::array_product_sum;
    use crate::average;
    use crate::binarysearch::binary_search;
    use crate::bincount;
    use crate::bintodec::bin_to_dec;
    use crate::bt_from_inorder_preorder;
    use crate::bubble_sort;
    use crate::build_closure_tag;
    use crate::build_heap;
    use crate::build_ip_addr;
    use crate::build_palindrome;
    use crate::caesar_crypt::caesar_crypt;
    use crate::camelcase::camelcase;
    use crate::can_increase_with_roundness;
    use crate::can_make_sum_a_with_k_num;
    use crate::capitalize;
    use crate::celcius_to_fahrenheit;
    use crate::century::century;
    use crate::char_count;
    use crate::chars_appearing_twice;
    use crate::chars_perm_list;
    use crate::chars_to_sorted_digits;
    use crate::christmas_tree::christmas_tree;
    use crate::clamp_value;
    use crate::classmark;
    use crate::clean_kth_bit::clean_kth_bit;
    use crate::code_breaker;
    use crate::combination;
    use crate::compare_and_swap;
    use crate::construct_min_height_bst;
    use crate::construct_square;
    use crate::contrast_check;
    use crate::cool_num_pair::cool_num_pair;
    use crate::cross_entropy;
    use crate::cumsum_except_last;
    use crate::cyclic_chars::cyclic_chars;
    use crate::decode_reverse_poland::decode_reverse_poland;
    use crate::dectobin::dectobin;
    use crate::deg_to_rad;
    use crate::dfs;
    use crate::diagonal::diagonal;
    use crate::different_squares;
    use crate::different_value_pairs;
    use crate::dns_water_torture;
    use crate::document_build::document_build;
    use crate::euclidean::euclidean;
    use crate::eval_tictactoe;
    use crate::even_num_sum::even_num_sum;
    use crate::factorial;
    use crate::fahrenheit_to_celcius;
    use crate::fibonacci;
    use crate::file_naming;
    use crate::find_closest_value;
    use crate::find_max_val_thread;
    use crate::find_successor;
    use crate::first_dupe_value;
    use crate::fizzbuzz::fizzbuzz;
    use crate::fsm_mealy;
    use crate::fsm_moor;
    use crate::geometric_progression;
    use crate::geometrical_mean;
    use crate::group_anagrams;
    use crate::group_by_class_range;
    use crate::group_by_key;
    use crate::guard_technique;
    use crate::hanoi_tower;
    use crate::harmonic_mean;
    use crate::helon_formula::helon_formula;
    use crate::human_and_cat_legs;
    use crate::image_stride;
    use crate::inorder_traversal;
    use crate::insertion_sort;
    use crate::invert_btree;
    use crate::ipv4;
    use crate::is_bst;
    use crate::is_mac_addr::is_mac_addr;
    use crate::is_palindrome::is_palindrome;
    use crate::is_two_array_similar;
    use crate::island_size;
    use crate::jump_height_in_frames;
    use crate::kadane::kadane;
    use crate::knapsack;
    use crate::kth_element_in_tree;
    use crate::l_r_value_sum_combo;
    use crate::lagrange_interpolation_polynominal;
    use crate::largest_adjacent_product;
    use crate::largest_number;
    use crate::lcs::lcs;
    use crate::least_data_eviction::least_data_eviction;
    use crate::least_factorial::least_factorial;
    use crate::least_greatest;
    use crate::least_lsd;
    use crate::length_linked_list;
    use crate::levenshtein_distance;
    use crate::loc_by_line_angle;
    use crate::linked_list;
    use crate::longest_pelindromic_substring;
    use crate::longest_substring_without_dupe;
    use crate::lower_upper_hinge;
    use crate::l0_norm;
    use crate::l1_norm;
    use crate::max_path_sum;
    use crate::max_sibling_product::max_sibling_product;
    use crate::max_subset_sum;
    use crate::max_with_lessdigit::max_with_lessdigit;
    use crate::maxnum_by_del_one_digit;
    use crate::mean;
    use crate::merge_2_lists;
    use crate::merge_sorted_linkedlist;
    use crate::min_breakdown_sum::min_breakdown_sum;
    use crate::min_passes_matrix_update;
    use crate::min_reward::min_reward;
    use crate::minimum_waiting_game;
    use crate::minesweeper;
    use crate::minmax_stack;
    use crate::mode;
    use crate::monotonic_array;
    use crate::most_frequent_digit_sum;
    use crate::n_steps;
    use crate::nega_posi_inversion;
    use crate::next_greater_element;
    use crate::node_distance_k;
    use crate::num_of_clans;
    use crate::num_of_paths;
    use crate::number_grouping;
    use crate::obtain_increasing_seq;
    use crate::octal_permission;
    use crate::octal_to_dec;
    use crate::oppsite_pos_in_circle;
    use crate::overlapping_intervals;
    use crate::partial_sum_count;
    use crate::pascal_triangle;
    use crate::peak_in_array;
    use crate::permutation;
    use crate::permutation_matrix;
    use crate::phone_mnemonic;
    use crate::postorder_traversal;
    use crate::powerset;
    use crate::prefix_sums;
    use crate::preorder_traversal::preorder_traversal;
    use crate::product_array_sort::product_array_sort;
    use crate::pseudo_rand;
    use crate::quarter;
    use crate::quick_sort;
    use crate::rad_to_deg;
    use crate::radix_sort::radix_sort;
    use crate::random_perm;
    use crate::ranking;
    use crate::reconstruct_bst_from_pre;
    use crate::remove_island;
    use crate::remove_nth_from_end;
    use crate::repeat_product::repeat_product;
    use crate::replace_mid_value;
    use crate::request_per_sec::request_per_sec;
    use crate::reverse_in_parenthiesis;
    use crate::reverse_poland_calc::reverse_poland_calc;
    use crate::reverse_stack;
    use crate::reverse_words;
    use crate::rgb_to_bgr;
    use crate::rgb_to_yuv;
    use crate::rightmost_diffbit::rightmost_diffbit;
    use crate::rightmost_samebit::rightmost_samebit;
    use crate::ring_buffer;
    use crate::runlength::runlength;
    use crate::same_bst;
    use crate::selection_sort::selection_sort;
    use crate::shapearea;
    use crate::shared::*;
    use crate::shorten_path;
    use crate::sigma_k::sigma_k;
    use crate::single_cycle_check;
    use crate::single_stroke;
    use crate::singly_linked_list_copy;
    use crate::singly_linked_list_reverse;
    use crate::smallest_difference;
    use crate::smallest_positive_product::smallest_positive_product;
    use crate::smallest_sum;
    use crate::sprite_index;
    use crate::softmax;
    use crate::softsign;
    use crate::sort_by_height;
    use crate::sort_stack;
    use crate::sorted_matrix_search;
    use crate::stack_copy_only_pop;
    use crate::stdev;
    use crate::storage_projection_for_sdhd;
    use crate::strange_bank;
    use crate::string_construction::string_construction;
    use crate::string_pattern::string_pattern;
    use crate::sturge_formula;
    use crate::subarray_sort;
    use crate::sudoku;
    use crate::suffix_trie;
    use crate::sum_of_arithmatic_progression;
    use crate::sum_of_consecutive_integers;
    use crate::sum_of_integers;
    use crate::sum_of_squared_deviation;
    use crate::sunset_view;
    use crate::swap_sibling::swap_sibling;
    use crate::tandem_bike;
    use crate::tandem_repeat::tandem_repeat;
    use crate::task_assignment;
    use crate::three_number_sort;
    use crate::three_sum;
    use crate::toggle_bit::toggle_bit;
    use crate::topological_sort;
    use crate::total_depth_sum_of_tree;
    use crate::total_sum_with_n_num;
    use crate::turn_commands;
    use crate::two_sum::two_sum;
    use crate::unconstructiable_number;
    use crate::valid_parentheses;
    use crate::valid_starting_city;
    use crate::validate_three_nodes;
    use crate::variance;
    use crate::waterarea::waterarea;
    use crate::xor_shift;
    use crate::yuv_to_rgb;
    use crate::z_score;
    use crate::zigzag_traversal;
    use float_cmp::approx_eq;
    use hashmap_macro::hashmap;
    use num::integer::gcd;
    use std::{
        collections::{BTreeSet, HashMap, HashSet, VecDeque},
    };
    use ndarray::{Array2, arr2};

    #[test]
    fn add_two_int_without_carry_test() {
        let a = 456;
        let b = 1734;
        assert_eq!(add_two_without_carry::exec(a, b), 1180);
    }

    #[test]
    fn all_longest_strings_test() {
        let rez = all_longest_strings::exec(&vec!["aba", "aa", "ad", "vcd", "aba"]);
        let expected = vec!["aba".to_string(), "vcd".to_string(), "aba".to_string()];
        assert_eq!(rez, expected);
    }

    #[test]
    fn almost_increasing_seq_false_case() {
        let s = [1, 3, 2, 1];
        assert_ne!(true, almost_increasing_seq::exec(&s));
    }

    #[test]
    fn almost_increasing_seq_true_case() {
        let s = [1, 2, 3, 4, 5, 5, 7, 6, 8];
        assert!(almost_increasing_seq::exec(&s));
    }

    #[test]
    fn alphabet_subseq_test() {
        assert_eq!(alhpabet_subseq::exec("effg"), false);
        assert_eq!(alhpabet_subseq::exec("cdce"), false);
        assert_eq!(alhpabet_subseq::exec("ace"), true);
        assert_eq!(alhpabet_subseq::exec("bxz"), true);
        assert_eq!(alhpabet_subseq::exec("abcdexyz"), true);
        assert_eq!(alhpabet_subseq::exec("abcdexmyz"), false);
    }

    #[test]
    fn appear_twice_test() {
        let expected: BTreeSet<char> = BTreeSet::from_iter(['b', 'd']);
        let rez = appear_twice::exec();
        assert_eq!(expected, rez);
    }

    #[test]
    fn applebox_test() {
        let n = 5;
        let expected = -15;
        assert_eq!(applebox::exec(n), expected);
    }

    #[test]
    fn arithmatic_progression_test() {
        assert_eq!(arithmatic_progression::exec(1, 5, 2), 5);
        assert_eq!(arithmatic_progression::exec(3, 5, 2), 9);
        assert_eq!(arithmatic_progression::exec(5, 1, 6), 25);
    }

    #[test]
    fn array_is_smooth_test() {
        assert_eq!(array_is_smooth::exec(&[7, 2, 2, 5, 10, 7]), true);
        assert_eq!(array_is_smooth::exec(&[-5, -5, 10]), false);
        assert_eq!(array_is_smooth::exec(&[7, 3, 7, 3, 7]), true);
        assert_eq!(array_is_smooth::exec(&[7, 2, 2, 5, 10, 2]), false);
        assert_eq!(array_is_smooth::exec(&[7, 3, 7, 3, 5]), false);
        assert_eq!(array_is_smooth::exec(&[3, 9, 3]), false);
        assert_eq!(array_is_smooth::exec(&[3, 3, 3]), true);
        assert_eq!(array_is_smooth::exec(&[2, 4]), false);
        assert_eq!(array_is_smooth::exec(&[4, 4]), false);
        assert_eq!(array_is_smooth::exec(&[5]), false);
    }

    #[test]
    fn array_product_sum_test() {
        let n = [5, 1, 4, 2];
        let p = &n[..];
        let r = array_product_sum::exec(p);
        assert_eq!(r, vec![8, 40, 10, 20]);
    }

    #[test]
    fn average_test() {
        let v = &[80_f32, 50_f32, 30_f32, 20_f32, 70_f32, 80_f32, 100_f32, 40_f32];
        assert_eq!(average::exec(v), 58.75);
    }

    #[test]
    fn bincount_test() {
        let a = [7, 3, 1, 10, 5, 0, 0, 1, 2, 1, 8, 1, 9, 1, 7];
        let b = bincount::exec(&a);
        let expected = vec![2, 5, 1, 1, 0, 1, 0, 2, 1, 1, 1];
        assert_eq!(b, expected);
    }

    #[test]
    fn binsearch_test_found() {
        let s = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
        let len = s.len();
        let b: Box<Vec<i32>> = Box::new(s);
        let r = binary_search::exec(&b, 33, 0, len - 1);
        assert_eq!(r, 3);
    }

    #[test]
    fn binsearch_test_notfound() {
        let s = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
        let len = s.len();
        let b: Box<Vec<i32>> = Box::new(s);
        let r = binary_search::exec(&b, 50, 0, len - 1);
        assert_eq!(r, -1);
    }

    #[test]
    fn bin_to_dec_test1() {
        let b = "10001010";
        let r = bin_to_dec::exec(b);
        assert_eq!(r, 138);
    }

    #[test]
    fn bin_to_dec_test2() {
        let b = "01001111";
        let r = bin_to_dec::exec(b);
        assert_eq!(r, 79);
    }

    #[test]
    fn bubble_sort_test() {
        let mut v = [7, 3, 10, 9, 3, 5, 11, 12, 6];
        let expected = [3, 3, 5, 6, 7, 9, 10 ,11, 12];
        bubble_sort::exec(&mut v);
        assert_eq!(v, expected);
    }

    #[test]
    fn build_btree_from_preorder_inorder_test() {
        let preorder = [3, 9, 20, 15, 7];
        let inorder = [9, 3, 15, 20, 7];
        match bt_from_inorder_preorder::exec(&preorder, &inorder) {
            Some(bn) => {
                let mut buf:Vec<i32> = Vec::new();
                traverse(bn, &mut buf);
                let mut expected:Vec<i32> = Vec::new();
                expected.extend_from_slice(&inorder);
                assert_eq!(buf, expected);
            },
            None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn build_closure_tag_test() {
        assert_eq!(build_closure_tag::exec("<button type='tako' disabled>neko desu.").unwrap(), "</button>".to_string());
        assert_eq!(build_closure_tag::exec("<button>neko desu.").unwrap(), "</button>");
        assert_eq!(build_closure_tag::exec("<button  >neko desu.").unwrap(), "</button>");
        assert!(build_closure_tag::exec("button type='neko'>").is_err());
    }

    #[test]
    fn build_heap_test() {
        let mut v = vec![15, 12, 3, 24, 31, 29, 21, 18, 25, 32];
        build_heap::exec(&mut v);
        let expected = vec![3, 12, 15, 18, 31, 29, 21, 24, 25, 32];
        assert_eq!(v, expected);
    }

    #[test]
    fn build_ip_addr_test() {
        let expected:HashSet<&str> = HashSet::from_iter(["1.9.216.80", "1.92.16.80", "1.92.168.0", "19.2.16.80", "19.2.168.0", "19.21.6.80", "19.21.68.0", "19.216.8.0", "192.1.6.80", "192.1.68.0", "192.16.8.0"]);
        let rez = build_ip_addr::exec("1921680").unwrap();
        rez.iter().all(|x|expected.contains(x.as_str()));
    }

    #[test]
    fn build_palindrome_test() {
        assert_eq!(build_palindrome::exec("abcdcba"), "abcdcba".to_string());
    }

    #[test]
    fn caesar_crypt_test() {
        let s = "TAkonbiz";
        let n = 3;
        assert_eq!("wdnrqelc", caesar_crypt::exec(s, n));
    }

    #[test]
    fn camelcase_test() {
        let c = camelcase::exec("taKOcHaN".to_string());
        assert_eq!(c, "Takochan".to_string());
    }

    #[test]
    fn can_increase_with_roundness_test() {
        assert_eq!(can_increase_with_roundness::exec(902200100), true);
        assert_eq!(can_increase_with_roundness::exec(11100), false);
    }

    #[test]
    fn can_make_sum_a_with_k_num_test() {
        assert_eq!(can_make_sum_a_with_k_num::exec(&[2, 3, 4, 5], 4, 14), true);
    }

    #[test]
    fn capitalize_test() {
        assert_eq!(capitalize::exec("ho ho ho"), "Ho ho ho".to_string());
        assert_eq!(capitalize::exec("HO HO HO"), "Ho ho ho".to_string());
        assert_eq!(capitalize::exec("hO Ho hO"), "Ho ho ho".to_string());
    }

    #[test]
    fn celcius_to_fahrenheit_test() {
        assert_eq!(celcius_to_fahrenheit::exec(25.), 77.);
        assert_eq!(celcius_to_fahrenheit::exec(12.), 53.6);
        assert_eq!(celcius_to_fahrenheit::exec(0.), 32.);
        assert_eq!(celcius_to_fahrenheit::exec(-3.), 26.6);
    }

    #[test]
    fn century_test1() {
        let n = 1905;
        let nr = century::exec(n);
        assert_eq!(nr, 20);
    }

    #[test]
    fn century_test2() {
        let n = 1700;
        let nr = century::exec(n);
        assert_eq!(nr, 17);
    }

    #[test]
    fn char_count_test() {
        let v = vec!["ABC", "ABDabd", "ZzZzZ", "MNN", "D", "ZA", "g"];
        let expected:HashMap<char, u32> = hashmap![
            'Z' => 4, 'A' => 3, 'B' => 2, 'D' => 2, 'b' => 1,
            'a' => 1, 'M' => 1, 'z' => 2, 'd' => 1, 'N' => 2,
            'C' => 1, 'g' => 1
        ];
        let rez = char_count::exec(v);
        assert_eq!(rez, expected);
    }

    #[test]
    fn chars_appearing_twice_exist_case_test() {
        let a = ['A', 'B', 'C'];
        let b = ['B', 'C', 'D'];
        let c = ['C', 'D', 'E'];
        let expected:HashSet<char> = HashSet::from_iter(['B', 'D'].iter().cloned());
        assert_eq!(chars_appearing_twice::exec(&a, &b, &c), expected);
    }

    #[test]
    fn chars_appearing_twice_nooverlap_case_test() {
        let a = ['A', 'B', 'C'];
        let b = ['D', 'E', 'F'];
        let c = ['G', 'H', 'I'];
        let expected:HashSet<char> = HashSet::new();
        assert_eq!(chars_appearing_twice::exec(&a, &b, &c), expected);
    }

    #[test]
    fn chars_appearing_twice_monochar_case_test() {
        let a = ['A', 'A', 'A'];
        let b = ['A', 'A', 'A'];
        let c = ['A', 'A', 'A'];
        let expected:HashSet<char> = HashSet::new();
        assert_eq!(chars_appearing_twice::exec(&a, &b, &c), expected);
    }

    #[test]
    fn chars_perm_list_test() {
        let expected:HashSet<&str> = HashSet::from_iter(
            [
                "ABCD", "ABDC", "ACBD", "ACDB", "ADBC", "ADCB", "BACD",
                "BADC", "BCAD", "BCDA", "BDAC", "BDCA", "CABD", "CADB",
                "CBAD", "CBDA", "CDAB", "CDBA", "DABC", "DBAC", "DBCA",
                "DCAB", "DCBA", "DACB"
            ]
        );
        let rez = chars_perm_list::exec("ABCD");
        let rs:HashSet<&str> = rez.iter().map(|s|s.as_str()).collect();
        assert_eq!(rs, expected);
    }

    #[test]
    fn chars_to_sorted_digits_test() {
        assert_eq!(chars_to_sorted_digits::exec("owoztneoer"), "012".to_string());
        assert_eq!(chars_to_sorted_digits::exec("fviefuro"), "45".to_string());
    }

    #[test]
    fn christmastree_test() {
        let t = [
            "       *  ",
            "     *    ",
            "***       ",
            "     *****",
            "   *******",
            "**********",
            " ***      ",
        ];
        let rez = christmas_tree::exec(&t);
        let rezstr: String = rez.join("\n");
        let expected = r"-----------
|    *    |
|    *    |
|   ***   |
|  *****  |
| ******* |
|*********|
|   ***   |
-----------";
        assert_eq!(rezstr, expected);
    }

    #[test]
    fn clamp_value_test() {
        assert_eq!(clamp_value::exec(72, 10, 100), 72);
        assert_eq!(clamp_value::exec(4, 10, 100), 10);
        assert_eq!(clamp_value::exec(105, 10, 100), 100);
    }

    #[test]
    fn classmark_test() {
        assert_eq!(classmark::exec(25., 35.), 42.5);
        assert_eq!(classmark::exec(0., 10.), 5.);
    }

    #[test]
    fn clean_kth_bit_test() {
        let n = 127;
        let k = 3;
        assert_eq!(clean_kth_bit::exec(n, k), 123);
    }

    #[test]
    fn code_breaker_test() {
        let ans = "708";
        assert_eq!(code_breaker::exec("212", ans), "___");
        assert_eq!(code_breaker::exec("549", ans), "___");
        assert_eq!(code_breaker::exec("756", ans), "H__");
        assert_eq!(code_breaker::exec("780", ans), "HBB");
        assert_eq!(code_breaker::exec("708", ans), "HHH");
    }

    #[test]
    fn combination_test() {
        let n = 8;
        let r = 3;
        assert_eq!(combination::exec(n, r), 56);
    }

    #[test]
    fn compare_and_swap_test() {
        let rez = std::panic::catch_unwind(||{
            compare_and_swap::run()
        });
        assert_ne!(rez.is_err(), true);
    }

    #[test]
    fn construct_square_simple_good_test() {
        let s = "aba".to_string();
        let rez = construct_square::exec(&s);
        assert_eq!(rez, 900);
    }

    #[test]
    fn construct_min_height_bst_test() {
        let v = [6, 3, 2, 1, 5, 4];
        let rez = construct_min_height_bst::exec(&v);
        let mut trace:Vec<i32> = Vec::new();
        traverse_pre(Box::new(rez), &mut trace);
        let expected = vec![3, 1, 2, 5, 4, 6];
        assert_eq!(trace, expected);
    }

    #[test]
    fn consturst_square_bad_case_test() {
        let s = "zzz".to_string();
        let rez = construct_square::exec(&s);
        assert_eq!(rez, -1);
    }

    #[test]
    fn construct_square_bignum_good_test() {
        let s = "aaaabbcde".to_string();
        let rez = construct_square::exec(&s);
        assert_eq!(rez, 999950884);
    }

    #[test]
    fn contrast_check_test() {
        assert_eq!(contrast_check::exec((60, 30, 20), (180, 200, 255)), true);
        assert_eq!(contrast_check::exec((100, 200, 220), (180, 200, 255)), false);
    }

    #[test]
    fn cool_num_pair() {
        let a = [4, 5, 6, 7, 8];
        let b = [8, 9, 10, 11, 12];
        assert_eq!(cool_num_pair::exec(&a, &b), 2);
    }

    #[test]
    fn cross_entropy_test() {
        let d = 1e-7;
        let y = vec![0.7, 0.1, 0.2];
        let t = vec![1., 0., 0.];
        let r = cross_entropy::exec(d, y, t);
        assert_eq!(r, 1.8538713);
    }

    #[test]
    fn cumsum_except_last_test() {
        assert_eq!(cumsum_except_last::exec(&[1,2,3,4,5]), 10);
    }

    #[test]
    fn cyclic_chars_test() {
        let n = 7;
        let s = "ABC";
        assert_eq!(cyclic_chars::exec(s, n), "ABCABCA");
    }

    #[test]
    fn decode_reverse_poland_test() {
        let s = "612+*8-";

        let expected = "6*(1+2)-8";
        assert_eq!(decode_reverse_poland::exec(s), expected);
    }

    #[test]
    fn deg_to_rad_test() {
        assert_eq!(deg_to_rad::exec(90.), 1.5707964);
        assert_eq!(deg_to_rad::exec(45.), 0.7853982);
        assert_eq!(deg_to_rad::exec(37.), 0.6457718);
    }

    #[test]
    fn dfs_test() {
        let node_i = MultiChildTreeNode::new("I".to_string());
        let node_j = MultiChildTreeNode::new("J".to_string());
        let node_k = MultiChildTreeNode::new("K".to_string());
        let node_e = MultiChildTreeNode::new("E".to_string());
        let mut node_g = MultiChildTreeNode::new("G".to_string());
        let mut node_f = MultiChildTreeNode::new("F".to_string());
        let node_h = MultiChildTreeNode::new("H".to_string());
        let mut node_b = MultiChildTreeNode::new("B".to_string());
        let node_c = MultiChildTreeNode::new("C".to_string());
        let mut node_d = MultiChildTreeNode::new("D".to_string());
        let mut node_a = MultiChildTreeNode::new("A".to_string());
        node_f.children = Some(vec![Box::new(node_i), Box::new(node_j)]);
        node_g.children = Some(vec![Box::new(node_k)]);
        node_b.children = Some(vec![Box::new(node_e), Box::new(node_f)]);
        node_d.children = Some(vec![Box::new(node_g), Box::new(node_h)]);
        node_a.children = Some(vec![Box::new(node_b), Box::new(node_c), Box::new(node_d)]);

        let mut buf:Vec<String> = Vec::new();
        dfs::exec(&Some(Box::new(node_a)), &mut buf);
        let expected_pre = vec!["A", "B", "E", "F", "I", "J", "C", "D", "G", "K", "H"];
        let expected: Vec<String>  = expected_pre.into_iter().map(|s|s.to_string()).collect();
        assert_eq!(buf, expected);
        
    }

    #[test]
    fn different_sq_test() {
        let v:Vec<Vec<i32>> = vec![
            vec![1, 2, 1],
            vec![2, 2, 2],
            vec![2, 2, 2],
            vec![1, 2, 3],
            vec![2 ,2, 1],
        ];
        assert_eq!(different_squares::exec(v), 6);
    }

    #[test]
    fn different_value_pairs_test() {
        let a = vec![5, 28, 14, 99, 17];
        let b = vec![5, 14, 28, 99, 16];
        let expected = vec![vec![28, 14, 17], vec![14, 28, 16]];
        assert_eq!(different_value_pairs::exec(a, b), expected);
    }

    #[test]
    #[should_panic]
    fn dns_water_torture_test() {
        dns_water_torture::exec("tanuki.com");
    }

    #[test]
    fn eval_tictactoe_test() {
        let s:Vec<&str> = vec![
            "OXXXOXXXO", "XXXXXXOOO", "XXXOOOXXX",
            "OOOXXXXXX", "XOXXOXXOX", "XXOXOXOXX",
            "OXXOXXOXX", "XXOXXOXXO"
        ];
        assert!(s.iter().all(|s|eval_tictactoe::exec(s)));
    }

    #[test]
    fn even_num_sum_match_case() {
        let n = 1230;
        assert!(even_num_sum::exec(n));
    }

    #[test]
    fn even_num_sum_unmatch_case() {
        let n = 124568;
        assert_eq!(false, even_num_sum::exec(n));
    }

    #[test]
    fn euclidean_test1() {
        let m = 128;
        let n = 72;
        let gcd = gcd(m, n);
        assert_eq!(euclidean::exec(m, n), gcd);
    }

    #[test]
    fn euclidean_test2() {
        let m = 72;
        let n = 128;
        let gcd = gcd(m, n);
        assert_eq!(euclidean::exec(m, n), gcd);
    }

    #[test]
    fn factorial_test() {
        let n = 10;
        assert_eq!(factorial::exec(n), 3628800);
    }

    #[test]
    fn fahrenheit_to_celcius_test() {
        let c1 = fahrenheit_to_celcius::exec(77.).round();
        assert_eq!(c1, 25.);
        let c2 = fahrenheit_to_celcius::exec(53.6).round();
        assert_eq!(c2, 12.);
        let c3 = fahrenheit_to_celcius::exec(32.0).round();
        assert_eq!(c3, 0.);
        let c4 = fahrenheit_to_celcius::exec(26.6).round();
        assert_eq!(c4, -3.);
    }

    #[test]
    fn file_naming_test() {
        let files = vec!["doc", "doc", "image", "doc(1)", "doc"];
        let expected = vec!["doc".to_string(), "doc(1)".to_string(), "image".to_string(), "doc(1)(1)".to_string(), "doc(2)".to_string()];
        assert_eq!(file_naming::exec(files), expected);
    }

    #[test]
    fn find_closest_value_exact_match_test() {
        let r = build_tree(&vec![10, 5, 2, 1, 15, 13, 14, 22]);
        assert_eq!(find_closest_value::exec(r, 12), 13);
    }

    #[test]
    fn find_closest_value_approx_match_test() {
        let r = build_tree(&vec![10, 5, 2, 1, 15, 13, 14, 22]);
        assert_eq!(find_closest_value::exec(r, 12), 13);
    }

    #[test]
    fn find_max_val_thread_test() {

        assert_eq!(find_max_val_thread::exec(&[72, 30, 1, 24, 9, 8, 132, -32, 50, 4]), 132);
        assert_eq!(find_max_val_thread::exec(&[22]), 22);
        assert_eq!(find_max_val_thread::exec(&[22, 72]), 72);
        assert_eq!(find_max_val_thread::exec(&[30, 10, 20]), 30);
        assert_eq!(find_max_val_thread::exec(&[40, 10, 50, 90, 10]), 90);
    }

    #[test]
    fn find_successor_test() {
        let n = build_tree(&vec![10, 5, 3, 7, 12, 2]);
        let r = find_successor::exec(*n, 7);
        assert!(r.is_some() && r.unwrap() == 10);
    }

    #[test]
    fn fibonacci_test() {
        let expected = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765];
        let rez:Vec<u64> = (1..=20).into_iter().map(|x|fibonacci::exec(x)).collect();
        assert_eq!(rez, expected);
    }

    #[test]
    fn first_dupe_value_test() {
        assert_eq!(first_dupe_value::exec(&[2, 1, 5, 2, 3, 3, 4]), 2);
        assert_eq!(first_dupe_value::exec(&[2, 1, 5, 9, 3, 7, 4]), -1);
        assert_eq!(first_dupe_value::exec(&[10, 1, 5, 9, 3, 7, 10]), 10);
    }

    #[test]
    fn test_fizzbuzz() {
        let n = 30;
        let rez = fizzbuzz::exec(n);
        let expected:String = "12fizz4buzzfizz78fizzbuzz11fizz1314fizzbuzz1617fizz19buzzfizz2223fizzbuzz26fizz2829fizzbuzz".to_string();
        assert_eq!(rez, expected);
    }

    #[test]
    fn fsm_mealy_test() {
        let init = "stop";
        let a = fsm_mealy::exec(init, "start").unwrap();
        assert_eq!(a, "running");
        let b = fsm_mealy::exec(a, "pause").unwrap();
        assert_eq!(b, "pause");
        let c = fsm_mealy::exec(b, "start").unwrap();
        assert_eq!(c, "running");
        let d = fsm_mealy::exec(c, "stop").unwrap();
        assert_eq!(d, "stop");
        let e = fsm_mealy::exec(d, "stop");
        match e {
            Ok(_) => panic!("not expected"),
            Err(e) => assert_eq!(e.downcast::<&str>().unwrap(), "not supported transition"),
        }
    }

    #[test]
    fn fsm_moor_test() {
        let init = "start";
        let a = fsm_moor::exec(init).unwrap();
        assert_eq!(a, "in_game");
        let b = fsm_moor::exec(a).unwrap();
        assert_eq!(b, "end");
        match fsm_moor::exec(b) {
            Ok(_) => panic!("not expected"),
            Err(e) => assert_eq!(e.downcast::<&str>().unwrap(), "no transition"),
        }
    }

    #[test]
    fn geometric_progression_test() {
        assert_eq!(geometric_progression::exec(1, 2, 3), 2);
        assert_eq!(geometric_progression::exec(3, 2, 3), 18);
        assert_eq!(geometric_progression::exec(4, 2, 3), 54);
        assert_eq!(geometric_progression::exec(5, 2, 3), 162);
    }

    #[test]
    fn geometrical_mean_test() {
        let v:Vec<f32> = vec![125_f32, 160_f32, 200_f32, 150_f32, 125_f32];
        assert_eq!(geometrical_mean::exec(v), 1.4962779);
    }

    #[test]
    fn group_anagrams_test() {
        let words = vec!["yo", "act", "flop", "tac", "foo", "cat", "oy", "olfp"];
        let expected = vec![vec!["yo", "oy"], vec!["flop", "olfp"], vec!["act", "tac", "cat"], vec!["foo"]];
        let rez = group_anagrams::exec(&words);
        assert!(expected.iter().all(|a|rez.contains(a)));
    }

    #[test]
    fn group_by_class_range_test() {
        assert_eq!(group_by_class_range::exec(&[20000, 239, 10001, 999999, 10000, 20566, 29999]), 11);
    }

    #[test]
    fn group_by_key_regular_case_test() {
        let l = vec![(1, 2), (1, 3), (3, 2), (4, 2), (4, 3)];
        let rez = group_by_key::exec(l, 1);
        assert_eq!(rez, vec![2, 3]);
    }

    #[test]
    fn group_by_key_no_key_case_test() {
        let l = vec![(1, 2), (1, 3), (3, 2), (4, 2), (4, 3)];
        let rez = group_by_key::exec(l, 5);
        assert_eq!(rez, vec![]);
    }

    #[test]
    fn guard_technique_test() {
        assert_eq!(guard_technique::exec(vec![3, 8, 11, 0, 9, 20, 0, 1, 8], 0), 22);
        assert_eq!(guard_technique::exec(vec![3, 8, 11, 1, 9, 20, 0, 1, 8], 0), 52);
        assert_eq!(guard_technique::exec(vec![3, 8, 11, 1, 9, 20, 3, 1, 8], 0), 64);
        assert_eq!(guard_technique::exec(vec![3, 8, 11, 1, 9, 20, 3, 1, 8], 1), 22);
    }

    #[test]
    fn hanoi_tower_test() {
        let rez = hanoi_tower::exec(4);
        let expected_raw = include_str!("../resource/hanoi_tower_test_expected.txt");
        let expected:Vec<String> = expected_raw.split_terminator("\n").into_iter().map(|s|s.to_string()).collect();
        assert_eq!(rez, expected);
    }

    #[test]
    fn harmonic_mean_test() {
        let s = vec![90_f32, 110_f32];
        assert_eq!(harmonic_mean::exec(&s), 99_f32);
    }

    #[test]
    fn helon_formula_test() {
        let a = 5.0;
        let b = 6.0;
        let c = 7.0;
        let expect = 14.6969385;
        let rez = approx_eq!(f32, expect, helon_formula::exec(a, b, c), ulps = 2);
        assert!(rez);
    }

    #[test]
    fn human_and_cat_legs_test() {
        assert_eq!(human_and_cat_legs::exec(6), vec![1, 3]);
        assert_eq!(human_and_cat_legs::exec(2), vec![1]);
    }

    #[test]
    fn image_stride_test() {
        let (w, h) = (512, 512);
        let (x, y) = (120, 247);
        let pix_size = 4;
        let rez = image_stride::exec(w, h, x, y, pix_size);
        let expected = (506336, 506339);
        assert_eq!(rez, expected);
    }

    #[test]
    fn inorder_trav_test() {
        let v = vec![5, 9, 2, 10, 1, 4];
        let r = inorder_traversal::exec(&v);
        let expected = vec![1, 2, 4, 5, 9, 10];
        assert_eq!(r, expected);
    }

    #[test]
    fn insertion_sort_test() {
        let mut v = [9, 15, 2, 7, 4, -5, 9, -3, 10, 8];
        insertion_sort::exec(&mut v);
        let expected = [-5, -3, 2, 4, 7, 8, 9, 9, 10, 15];
        assert_eq!(v, expected);
    }

    #[test]
    fn invert_btree_test() {
        let mut n = build_tree(&vec![100, 50, 10, 60, 200, 170, 300, 250, 500]);
        invert_btree::exec(&mut n);
        let mut v:Vec<i32> = Vec::new();
        traverse(n, &mut v);
        let expected:Vec<i32> = vec![500, 300, 250, 200, 170, 100, 60, 50, 10];
        assert_eq!(v, expected);
    }

    #[test]
    fn ipv4_valid_case_test() {
        let validones = vec!["192.168.0.1", "0.0.0.0", "255.255.255.255",
        "129.20.38.0", "240.240.240.240", "1.2.3.4", "10.20.30.40", "99.99.99.99",
        "127.0.0.1"];
        assert!(validones.iter().all(|s|ipv4::exec(s)));
    }

    #[test]
    fn ipv4_invalid_case_test() {
        let invalidones = vec!["-1.2.3.4", "256.2.1.8", "79.79.79.790", "192.168.8", "192.168.256.8", "260.2.2.100",
        "50", "0.0.0.256", "1000.100000.1000000000.10", "a.b.c.d", "192.01.25.33", "08.10.2.25", "0001.0004.0008.0002"];
        assert!(invalidones.iter().all(|s|!ipv4::exec(s)));
    }

    #[test]
    fn is_bst_test() {
        let v = vec![5, 9, 2, 10, 1, 4];
        assert!(is_bst::exec(&v));
    }

    #[test]
    fn is_mac_addr_good() {
        let mac = "00-1B-63-84-45-E6";
        assert_eq!(is_mac_addr::exec(mac), true);
    }

    #[test]
    fn is_palindrome_good_evencase() {
        let s = "abccba";
        assert_eq!(is_palindrome::exec(&s.to_string()), true);
    }

    #[test]
    fn is_palindrome_good_oddcase() {
        let s = "abcba";
        assert_eq!(is_palindrome::exec(&s.to_string()), true);
    }

    #[test]
    fn is_palindrome_badcase() {
        let s = "abaya";
        assert_eq!(is_palindrome::exec(&s.to_string()), false);
    }

    #[test]
    fn is_mac_addr_bad() {
        let mac = "00-1B-63-84-45-Z6";
        assert_eq!(is_mac_addr::exec(mac), false);
    }

    #[test]
    fn is_two_array_similar_test() {
        assert_eq!(is_two_array_similar::exec(&[1, 2, 3], &[2, 1, 3]), true);
        assert_eq!(is_two_array_similar::exec(&[1, 2, 3], &[1, 2, 3]), true);
        assert_eq!(is_two_array_similar::exec(&[1, 2, 2], &[2, 1, 1]), false);
    }

    #[test]
    fn island_size_test() {
        let m = arr2(&[
            [1, 0, 0, 1, 0],
            [1, 0, 1, 0, 0],
            [0, 0, 1, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 1, 1, 0]
        ]);
        let expected = vec![1, 2, 2, 2, 5];
        let mut rez = island_size::exec(&m);
        rez.sort();
        assert_eq!(rez, expected);
    }

    #[test]
    fn jump_height_in_frames_test() {
        approx_eq!(f32, jump_height_in_frames::exec(0, 40, 120), 0_f32);
        approx_eq!(f32, jump_height_in_frames::exec(20, 40, 120), 120_f32);
    }

    #[test]
    fn kadane_test() {
        let v = vec![3, 5, -9, 1, 3, -2, 3, 4, 7, 2, -9, 6, 3, 1, -5, 4];
        assert_eq!(19, kadane::exec(v));
    }

    #[test]
    fn knapsack_test() {
        assert_eq!(knapsack::exec(vec![2, 1, 3, 2, 1, 5], vec![3, 2, 6, 1, 3, 85], 9), 94);
    }

    #[test]
    fn kth_element_in_tree_test() {
        let v:Vec<i32> = vec![15, 5, 2, 1, 3, 5, 20, 17, 22];
        let n = build_tree(&v);
        let rez = kth_element_in_tree::exec(n, 3);
        assert_eq!(rez, 17);
    }

    #[test]
    fn l_r_value_sum_combo_test() {
        assert_eq!(l_r_value_sum_combo::exec(6, 4, 2), 2);
        assert_eq!(l_r_value_sum_combo::exec(6, 2, 4), 2);
        assert_eq!(l_r_value_sum_combo::exec(7, 2, 4), 1);
    }

    #[test]
    fn lagrange_interpolation_polynominal_test() {

        let points = vec![FPoint::new(0.0, 0.8),
        FPoint::new(1.0, 3.1),
        FPoint::new(3.0, 4.5),
        FPoint::new(6.0, 3.9),
        FPoint::new(7.0, 2.8)];
        let rez = lagrange_interpolation_polynominal::exec(&points, 0_f32, 7.1_f32, 0.5_f32);
        let expected:Vec<FPoint> = vec![FPoint::new(0.0, 0.8),
        FPoint::new(0.5, 2.1527407), FPoint::new(1.0, 3.1), FPoint::new(1.5, 3.7357361),
        FPoint::new(2.0, 4.1396823), FPoint::new(2.5, 4.3773437), FPoint::new(3.0, 4.5),
        FPoint::new(3.5, 4.5447054), FPoint::new(4.0, 4.534286), FPoint::new(4.5, 4.4773445),
        FPoint::new(5.0, 4.368254), FPoint::new(5.5, 4.1871653), FPoint::new(6.0, 3.9),
        FPoint::new(6.5, 3.458455), FPoint::new(7.0, 2.8)];
    }

    #[test]
    fn largest_adjacent_product_many_elem_test() {
        let v = [3, 6, -2, -5, 7, 3];
        assert_eq!(largest_adjacent_product::exec(&v), 21);
    }

    #[test]
    fn largest_adjacent_product_single_elem_test() {
        let v= [0];
        assert_eq!(largest_adjacent_product::exec(&v), 0);
    }

    #[test]
    fn largest_adjacent_product_no_elem_test() {
        let v:&[i32] = &[];
        let maybe_e = std::panic::catch_unwind(||{
            largest_adjacent_product::exec(v);
        });
        assert!(maybe_e.is_err());
    }

    #[test]
    fn largest_number_test() {
        let expected:Vec<u64> = vec![0, 9, 99, 999, 9999, 99999, 999999,
        9999999, 99999999, 999999999, 9999999999];
        for i in 1..=10 {
            let r:u64 = largest_number::exec(i);
            assert_eq!(r, expected[i as usize]);
        }
    }

    #[test]
    fn lcs_test1() {
        let m = "abcde";
        let n = "acbef";
        assert_eq!(lcs::exec(m, n), 3);
    }

    #[test]
    fn least_data_eviction_test() {
        let seq = vec![12, 4, 5, 19, 20, 10, 6, 2, 1, 33, 22, 25, 16, 7, 4, 20, 15, 30, 2, 7];
        let rez = least_data_eviction::exec(&seq);
        assert_eq!(rez.len(), 3);
        assert_eq!(rez, vec![25, 30, 33]);
    }

    #[test]
    fn least_factorial_test() {
        let n = 17;
        assert_eq!(least_factorial::exec(n), 24);
    }

    #[test]
    fn least_greatest_test() {
        use least_greatest::IntNum;
        let v:Vec<IntNum> = vec![IntNum::new(8), IntNum::new(12), IntNum::new(25), IntNum::new(3), IntNum::new(50)];
        let (least, greatest) = least_greatest::exec(&v);
        assert_eq!(least, 3);
        assert_eq!(greatest, 50);
    }

    #[test]
    fn least_lsd_test() {
        assert_eq!(least_lsd::exec(872), 2);
        assert_eq!(least_lsd::exec(1111), 1);
        assert_eq!(least_lsd::exec(1234), 4);
        assert_eq!(least_lsd::exec(5), 5);
        assert_eq!(least_lsd::exec(2208893445), 5);
    }

    #[test]
    fn length_linked_list_test() {
        let v = vec![1,2,3,4,5];
        assert_eq!(length_linked_list::exec(&v), 5);
    }

    #[test]
    fn levenshtein_distance_test() {
        let s = "abc";
        let t = "yabd";
        let diff = levenshtein_distance::exec(s, t);
        assert_eq!(diff, 2);
    }

    #[test]
    fn linked_list_forward_trav_test() {
        let v = &[1, 2, 3, 4, 5];
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(linked_list::exec(v), expected);
    }

    #[test]
    fn loc_by_line_angle_test() {
        assert_eq!(loc_by_line_angle::exec(15., 90.), 90.);
        assert_eq!(loc_by_line_angle::exec(15., 45.), 48.63961);
    }

    #[test]
    fn longest_pelindromic_substring_evencase_test() {
        let s = "abaxyzzyxf";
        let expected = "xyzzyx";
        assert_eq!(longest_pelindromic_substring::exec(s), expected);
    }

    #[test]
    fn longest_pelindromic_substring_oddcase_test() {
        let s = "ababrrbzaxoxazxn";
        let expected = "zaxoxaz";
        assert_eq!(longest_pelindromic_substring::exec(s), expected);
    }

    #[test]
    fn longest_substring_without_dupe_test() {
        let expected = "nemvjsza";
        let rez = longest_substring_without_dupe::exec("akenemvjszazq");
        assert_eq!(rez, expected);
    }

    #[test]
    fn lower_upper_hinge_test() {
        assert_eq!(lower_upper_hinge::exec(&[12, 13, 14, 15, 17]), (13_f32, 15_f32));
        assert_eq!(lower_upper_hinge::exec(&[1, 2, 3]), (1_f32, 3_f32));
        assert_eq!(lower_upper_hinge::exec(&[1, 2, 3, 4, 5, 6, 7, 8, 9]), (3_f32, 7_f32));
        assert_eq!(lower_upper_hinge::exec(&[1, 2]), (1_f32, 2_f32));
        assert_eq!(lower_upper_hinge::exec(&[1, 2, 3, 4]), (1.5, 3.5));
        assert_eq!(lower_upper_hinge::exec(&[1, 2, 3, 4, 5, 6, 7, 8]), (2.5, 6.5));
    }

    #[test]
    fn l0_norm_test() {
        assert_eq!(l0_norm::exec(&[3, -1, -2, 0, 0, 4]), 4);
        assert_eq!(l0_norm::exec(&[3, -1, -2, 3, 1, 4]), 6);
        assert_eq!(l0_norm::exec(&[0, 0, 0]), 0);
    }

    #[test]
    fn l1_norm_test() {
        let v = [1.2, 0.8, -0.5, 0., 2., -1.5];
        assert_eq!(l1_norm::exec(&v), 6.0);
    }

    #[test]
    fn max_path_sum_test() {
        use std::collections::VecDeque;
        let mut root = Box::new(TreeNode { value: 1, left: None, right: None});
        let mut op1 = VecDeque::from_iter(["left"]);
        add_node_not_balanced(&mut root, 2, &mut op1);
        let mut op2 = VecDeque::from_iter(["left", "left"]);
        add_node_not_balanced(&mut root, 4, &mut op2);
        let mut op3 = VecDeque::from_iter(["left", "right"]);
        add_node_not_balanced(&mut root, 5, &mut op3);
        let mut op4 = VecDeque::from_iter(["right"]);
        add_node_not_balanced(&mut root, 3, &mut op4);
        let mut op5 = VecDeque::from_iter(["right", "left"]);
        add_node_not_balanced(&mut root, 6, &mut op5);
        let mut op6 = VecDeque::from_iter(["right", "right"]);
        add_node_not_balanced(&mut root, 7, &mut op6);

        let rez = max_path_sum::exec(*root);
        assert_eq!(rez, 18);
    }

    #[test]
    fn max_sibling_product_test() {
        let s = vec![3, 6, -2, -5, 7, 3];
        let r = max_sibling_product::exec(s);
        assert_eq!(r, 21);
    }

    #[test]
    fn max_subset_sum_test() {
        let a = [75, 105, 120, 75, 90, 135];
        assert_eq!(max_subset_sum::exec(&a), 330);
    }

    #[test]
    fn max_with_lessdigit_test1() {
        let n = 1001;
        let maxv = max_with_lessdigit::exec(n);
        assert_eq!(maxv, 101);
    }

    #[test]
    fn max_with_lessdigit_test2() {
        let n = 597;
        let maxv = max_with_lessdigit::exec(n);
        assert_eq!(maxv, 97);
    }

    #[test]
    fn maxnum_by_del_one_digit_test() {
        assert_eq!(maxnum_by_del_one_digit::exec(152), 52);
        assert_eq!(maxnum_by_del_one_digit::exec(1001), 101);
    }

    #[test]
    fn mean_test() {
        let v = &mut [540_f32, 280_f32, 3000_f32, 540_f32, 480_f32];
        let v2 = &mut [100_f32, 110_f32, 150_f32, 180_f32, 300_f32, 600_f32];
        assert!(approx_eq!(f32, mean::exec(v), 540_f32));
        assert!(approx_eq!(f32, mean::exec(v2), 165_f32));
    }

    #[test]
    fn merge_2_lists_test() {
        let mut a = [7, 3, 10, 2, 10, 25, 9, 11];
        let mut b = [3, 4, 19, 21, 15];
        let c = merge_2_lists::exec(&mut a, &mut b);
        let expected = vec![2, 3, 3, 4, 7, 9, 10, 10, 11, 15, 19, 21, 25];
        assert_eq!(c, expected);
    }

    #[test]
    fn merge_sorted_linkedlist_test() {
        let mut l1 = vec![2, 6, 7, 8];
        let mut l2 = vec![1, 3, 4, 5, 9, 10];
        merge_sorted_linkedlist::exec(&mut l1, &mut l2);
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(l2, expected);
    }

    #[test]
    fn min_breakdown_sum_test1() {
        let n:u32 = 240;
        let rez = min_breakdown_sum::exec(n);
        assert_eq!(rez, 4);
    }

    #[test]
    fn min_breakdown_sum_test2() {
        let n:u32 = 808;
        let rez = min_breakdown_sum::exec(n);
        assert_eq!(rez, 14);
    }

    #[test]
    fn min_passes_matrix_update_test() {
        let mut m:Array2<i32> = arr2(&[
            [0, -1, -3, 2, 0],
            [1, -2, -5, -1, -3],
            [3, 0, 0, -4, -1]
        ]);
        assert_eq!(min_passes_matrix_update::exec(&mut m), 3);
    }

    #[test]
    fn min_reward_test() {
        let scores = [8, 4, 2, 1, 3, 6, 7, 9, 5];
        assert_eq!(min_reward::exec(&scores), 25);
    }

    #[test]
    fn minimum_waiting_game_test() {
        let v = &[5, 1, 4];
        assert_eq!(minimum_waiting_game::exec(v), 5);
    }

    #[test]
    fn minesweeper_test() {
        let m:Array2<bool> = arr2(&[
            [true, false, false],
            [false, true, false],
            [false, false, false]
        ]);
        let rez:Array2<i32> = minesweeper::exec(&m);
        let rezv = rez.view();
        let expected:Array2<i32> = arr2(&[
            [1, 2, 1],
            [2, 1, 1],
            [1, 1, 1],
        ]);
        let ev = expected.view();
        assert_eq!(rezv, ev);
    }

    #[test]
    fn minmax_stack_test() {
        let maybe_err = std::panic::catch_unwind(||{
            minmax_stack::run();
        });
        assert_ne!(maybe_err.is_err(), true);
    }

    #[test]
    fn mode_test() {
        let t = vec![3, 1, 6, 1, 5, 8, 1, 8, 10, 11];
        assert_eq!(mode::exec(&t), 1);
    }

    #[test]
    fn monotonic_array_test() {
        let a = [3, 7, 10, 25, 50, 100, 150];
        let b = [100, 72, 55, 40, 32, 10, 3, 1];
        let c = [150, 125, 72, 138, 50, 62, 22];
        let d = [1, 1, 1, 1, 2, 3, 5, 10];
        let e = [10, 10, 10, 7, 4, 3, 1];
        let f = [5, 5, 5, 5, 5, 5, 5];
        assert_eq!(monotonic_array::exec(&a), Some(true));
        assert_eq!(monotonic_array::exec(&b), Some(true));
        assert_eq!(monotonic_array::exec(&c), Some(false));
        assert_eq!(monotonic_array::exec(&d), Some(true));
        assert_eq!(monotonic_array::exec(&e), Some(true));
        assert_eq!(monotonic_array::exec(&f), None);
    }

    #[test]
    fn most_frequent_digit_sum_test() {
        assert_eq!(most_frequent_digit_sum::exec(88), 9);
        assert_eq!(most_frequent_digit_sum::exec(994), 9);
        assert_eq!(most_frequent_digit_sum::exec(239), 9);
        assert_eq!(most_frequent_digit_sum::exec(1), 1);
        assert_eq!(most_frequent_digit_sum::exec(99999), 18);
    }

    #[test]
    fn n_steps_test() {
        assert_eq!(n_steps::exec(4, 2), 5);
    }

    #[test]
    fn nega_posi_inversion_test() {
        assert_eq!(nega_posi_inversion::exec(72), 183);
        assert_eq!(nega_posi_inversion::exec(0), 255);
        assert_eq!(nega_posi_inversion::exec(255), 0);
        assert_eq!(nega_posi_inversion::exec(128), 127);
    }

    #[test]
    fn next_greater_element_test() {
        let a = &[2, 5, -3, -4, 6, 7, 2];
        let expected = vec![5, 6, 6, 6, 7, -1, 5];
        assert_eq!(next_greater_element::exec(a), expected);
    }

    #[test]
    fn node_distance_k_test() {
        let mut root = Box::new(TreeNode { value: 1, left: None, right: None});
        let mut op1 = VecDeque::from_iter(["left"]);
        add_node_not_balanced(&mut root, 2, &mut op1);
        let mut op2 = VecDeque::from_iter(["right"]);
        add_node_not_balanced(&mut root, 3, &mut op2);
        let mut op3 = VecDeque::from_iter(["left", "left"]);
        add_node_not_balanced(&mut root, 4, &mut op3);
        let mut op4 = VecDeque::from_iter(["left", "right"]);
        add_node_not_balanced(&mut root, 5, &mut op4);
        let mut op5 = VecDeque::from_iter(["right", "right"]);
        add_node_not_balanced(&mut root, 6, &mut op5);
        let mut op6 = VecDeque::from_iter(["right", "right", "left"]);
        add_node_not_balanced(&mut root, 7, &mut op6);
        let mut op7 = VecDeque::from_iter(["right", "right", "right"]);
        add_node_not_balanced(&mut root, 8, &mut op7);
        let rez:HashSet<i32> = node_distance_k::exec(root, 3, 2).into_iter().collect();
        assert_eq!(rez, HashSet::from_iter([2, 7, 8]));
    }

    #[test]
    fn num_of_clans_test() {
        let divisors = vec![2, 3, 4];
        let k = 6;
        let rez = num_of_clans::exec(divisors, k);
        assert_eq!(rez, 5);
    }

    #[test]
    fn num_of_paths_test() {
        let w = 5;
        let h = 4;
        let r = num_of_paths::exec(w, h);
        assert_eq!(r, 35);
    }

    #[test]
    fn number_grouping_test() {
        let a = vec![10000, 20000, 30000, 40000, 50000, 60000, 10000, 120000, 150000, 200000, 300000, 1000000, 10000000, 100000000, 10000000];
        assert_eq!(number_grouping::exec(&a), 28);
    }

    #[test]
    fn obtain_increasing_seq_normal_case_test() {
        let v = [1, 1, 1];
        assert_eq!(obtain_increasing_seq::exec(&v), 3);
    }

    #[test]
    fn obtain_increasing_seq_already_done_case_test() {
        let v = [1, 2, 3];
        assert_eq!(obtain_increasing_seq::exec(&v), 0);
    }

    #[test]
    fn octal_permission_test() {

        let expected:HashMap<String, String> = hashmap!(
            "Owner".to_string() => "rwx".to_string(),
            "Group".to_string() => "r".to_string(),
            "Other".to_string() => "w".to_string()
        );
        assert_eq!(octal_permission::exec("742"), expected);
    }

    #[test]
    fn octal_to_dec_test() {
        assert_eq!(octal_to_dec::exec(127), 87);
        assert_eq!(octal_to_dec::exec(5351), 2793);
        assert_eq!(octal_to_dec::exec(7), 7);
        assert_eq!(octal_to_dec::exec(8), 8);
        assert_eq!(octal_to_dec::exec(16), 14);
    }

    #[test]
    fn opposite_num_in_circle_test() {
        let n = 10;
        let f = 2;
        assert_eq!(oppsite_pos_in_circle::exec(n, f), 7);
    }

    #[test]
    fn overlapping_intervals_test() {
        use overlapping_intervals::Pair;
        let n:Vec<Pair> = vec![Pair::new(1, 2), Pair::new(3, 5), Pair::new(4, 7),
            Pair::new(6, 8), Pair::new(9, 10)];
        let rez = overlapping_intervals::exec(n);
        let expected:Vec<Pair> = vec![Pair::new(1, 2), Pair::new(3, 8), Pair::new(9, 10)];
        assert_eq!(rez, expected);
    }

    #[test]
    fn lcs_test2() {
        let m = "pirikapirirara";
        let n = "poporinapeperuto";
        assert_eq!(lcs::exec(m, n), 6);
    }

    #[test]
    fn radix_sort_test() {
        let mut v = vec![8762, 654, 3008, 345, 87, 65, 234, 12, 2];
        radix_sort::exec(&mut v);
        assert_eq!(vec![2, 12, 65, 87, 234, 345, 654, 3008, 8762], v);
    }

    #[test]
    fn partial_sum_count_test() {
        let s = partial_sum_count::exec(&[1, 2, 1, 3, 2], 4);
        assert_eq!(s, 5);
    }

    #[test]
    fn pascal_tr_test() {
        let n = 5;
        let expected = r"1
1 1
1 2 1
1 3 3 1
1 4 6 4 1";
        let rez = pascal_triangle::exec(n);
        assert_eq!(rez.trim(), expected);
    }

    #[test]
    fn peak_in_array_test() {
        let v = [1, 2, 3, 3, 4, 0, 10, 6, 5, -1, -3, 2, 3];
        assert_eq!(peak_in_array::exec(&v), 6);
    }

    #[test]
    fn perm_test() {
        let n = 8;
        let r = 6;
        assert_eq!(permutation::exec(n, r), 20160);
    }

    #[test]
    fn permutation_matrix_test() {
        let siz = 5;
        let m:Array2<i32> = permutation_matrix::exec(siz);
        for i in 0..siz {
            let mut cnt = 0;
            for j in 0..siz {
                if m[[i, j]] == 1 {
                    cnt += 1;
                }
            }
            assert_eq!(cnt, 1);
        }
    }

    #[test]
    fn phone_mnemonic_test() {
        use hmap::hmap;
        use std::collections::HashMap;
        let table:HashMap<char, String> = hmap!(
            '2' => "abc".to_string(),
            '3' => "def".to_string(),
            '4' => "ghi".to_string(),
            '5' => "jkl".to_string(),
            '6' => "mno".to_string(),
            '7' => "pqrs".to_string(),
            '8' => "tuv".to_string(),
            '9' => "wxyz".to_string()
        );
        let rez_raw = phone_mnemonic::exec("1905", &table);
        let rez:Vec<&str> = rez_raw.iter().map(|x|x.as_str()).collect();
        let expected = vec!["1w0j", "1w0k", "1w0l", "1x0j", "1x0k", "1x0l", "1y0j", "1y0k", "1y0l", "1z0j", "1z0k", "1z0l"];
        assert_eq!(rez, expected);
    }

    #[test]
    fn postorder_traversal_test() {
        let rez = postorder_traversal::exec(vec![5, 9, 2, 10, 1, 4]);
        let expected = vec![1, 4, 2, 10, 9, 5];
        assert_eq!(rez, expected);
    }

    #[test]
    fn powerset_test() {
        let expected:Vec<Vec<i32>> = vec![vec![], vec![1], vec![2], vec![3], vec![1, 2], vec![1, 3], vec![2, 3], vec![1, 2, 3]];
        let rez = powerset::exec(vec![1, 2, 3]);
        for r in rez {
            assert!(expected.contains(&r));
        }
    }

    #[test]
    fn prefix_sums_test() {
        let v = [1, 2, 3];
        let expected = vec![1, 3, 6];
        assert_eq!(prefix_sums::exec(&v), expected);
    }

    #[test]
    fn preorder_trav_test() {
        let v: Vec<i32> = vec![5, 9, 2, 10, 1, 4];
        let r: Vec<i32> = preorder_traversal::exec(v);
        assert_eq!(r, vec![5, 2, 1, 4, 9, 10]);
    }

    #[test]
    fn pseudo_rand_ok_test() {
        let a = 109;
        let c=  1021;
        let m = 256;
        let x0 = 13;
        let v:Vec<i32> = pseudo_rand::exec(a, c, m, x0, 20);
        assert!(v.iter().all(|x|*x >= 0 && *x <= m));
    }

    #[test]
    fn pseudo_rand_fail_test() {
        let rez = std::panic::catch_unwind(||{
            let a = 106;
            let c=  1021;
            let m = 256;
            let x0 = 13;
            pseudo_rand::exec(a, c, m, x0, 20);
        });
        assert!(rez.is_err());
    }

    #[test]
    fn quarter_test() {
        let expected = [0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4];
        for m in 1..=12 {
            let r = quarter::exec(m as u8);
            assert_eq!(r, expected[m]);
        }
    }

    #[test]
    fn quick_sort_test() {
        let mut v = vec![7, 1, 2, 5, 10, 20, 15, 13, 9, 5, 4];
        let expected = vec![1, 2, 4, 5, 5, 7, 9, 10, 13, 15, 20];
        let len = v.len();
        quick_sort::exec(&mut v, 0, len - 1);
        assert_eq!(v, expected);
    }

    #[test]
    fn rad_to_deg_test() {
        assert_eq!(rad_to_deg::exec(1.2), 68.75494);
        assert_eq!(rad_to_deg::exec(2.42), 138.65578);
        assert_eq!(rad_to_deg::exec(0.79), 45.263664);
    }

    #[test]
    fn rand_perm_test() {
        let o:Vec<i32> = (0..=10).collect();
        let mut v:Vec<i32> = (0..=10).collect();
        random_perm::exec(&mut v);
        assert_ne!(v, o);
    }

    #[test]
    fn ranking_test() {
        let scores = vec![56, 25, 67, 88, 100, 61, 55, 67, 76, 56];
        let expected:Vec<(i32, u32)> = vec![(56, 7), (25, 10), (67, 4), (88, 2), (100, 1), (61, 6), (55, 9), (67, 4), (76, 3), (56, 7)];
        assert_eq!(ranking::exec(&scores), expected);
    }

    #[test]
    fn reconstruct_bst_from_preorder_test() {
        let v:Vec<i32> = vec![10, 4, 2, 1, 5, 17, 19, 18];
        let Some(rez) = reconstruct_bst_from_pre::exec(&v) else {
            panic!("failed");
        };
        let mut trace:Vec<i32> = Vec::new();
        traverse_pre(rez, &mut trace);
        assert_eq!(v, trace);
    }

    #[test]
    fn remove_island_test() {
        let mut matrix = arr2(&[
            [1, 0, 0, 0, 0, 0],
            [0, 1, 0, 1, 1, 1],
            [0, 0, 1, 0, 1, 0],
            [1, 1, 0, 0, 1, 0],
            [1, 0, 1, 1, 0, 0],
            [1, 0, 0, 0, 0, 1],
        ]);
        remove_island::exec(&mut matrix);
        let expected = arr2(&[
            [1, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1],
            [0, 0, 0, 0, 1, 0],
            [1, 1, 0, 0, 1, 0],
            [1, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 1]
        ]);
        assert_eq!(matrix.view(), expected.view());
    }

    #[test]
    fn remove_nth_node_from_last_test() {
        let mut n = *build_singly_linkedlist(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        remove_nth_from_end::exec(&mut n, 4);
        let mut v:Vec<i32> = Vec::new();
        n.trav_from(&mut v);
        let expected = vec![1, 2, 3, 4, 5, 7, 8, 9];
        assert_eq!(v, expected);
    }

    #[test]
    fn repeat_product_test() {
        let n = 16;
        assert_eq!(repeat_product::exec(n), 9);
    }

    #[test]
    fn replace_mid_value_test() {
        assert_eq!(replace_mid_value::exec(&[7,2,2,5,10,7]), &[7, 2, 7, 10, 7]);
        assert_eq!(replace_mid_value::exec(&[-5, -5, 10]), &[-5, -5, 10]);
        assert_eq!(replace_mid_value::exec(&[10]), &[10]);
        assert_eq!(replace_mid_value::exec(&[10, 20]), &[30]);
    }

    #[test]
    fn req_per_sec_test() {
        let n = 17;
        let rez = request_per_sec::exec(n);
        println!("{}-{}", rez, 928);
        assert!(true);
    }

    #[test]
    fn reverse_in_parenthiesis_ok_case_test() {
        assert_eq!(reverse_in_parenthiesis::exec("(bar)").unwrap().as_str(), "rab");
        assert_eq!(reverse_in_parenthiesis::exec("foo(bar)baz").unwrap().as_str(), "foorabbaz");
        assert_eq!(reverse_in_parenthiesis::exec("foo(bar)baz(blim)").unwrap().as_str(), "foorabbazmilb");
        assert_eq!(reverse_in_parenthiesis::exec("foo(bar(baz))blim").unwrap().as_str(), "foobazrabblim");
    }

    #[test]
    fn reverse_in_parenthiesis_fail_case_test() {
        assert!(reverse_in_parenthiesis::exec("bar)").is_err());
        assert!(reverse_in_parenthiesis::exec("(bar").unwrap().as_str() == "(bar");
    }

    #[test]
    fn reverse_poland_test1() {
        let s = "12+3+4+";
        assert_eq!(reverse_poland_calc::exec(s), 10_f32);
    }

    #[test]
    fn reverse_poland_test2() {
        let s = "374/-8*";
        assert_eq!(reverse_poland_calc::exec(s), 10_f32);
    }

    #[test]
    fn reverse_stack_test() {
        let mut buf:Vec<i32> = Vec::new();
        let mut s:Vec<i32> = Vec::from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let expected:Vec<i32> = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        reverse_stack::exec(&mut s, &mut buf);
        assert_eq!(buf, expected);
        assert!(s.is_empty());
    }

    #[test]
    fn reverse_words_test() {
        let r = "nekochan kawaii to omoimasenka? soudesu.";
        let expected:String = "soudesu. omoimasenka? to kawaii nekochan".to_string();
        assert_eq!(reverse_words::exec(r), expected);
    }

    #[test]
    fn rgb_to_bgr_test() {
        let pix = [255, 0, 100];
        assert_eq!(rgb_to_bgr::exec(&pix), [100, 0, 255]);
    }

    #[test]
    fn rgb_to_yuv_test() {
        let (r, g, b) = (255, 192, 128);
        assert_eq!(rgb_to_yuv::exec(r, g, b), (203.7705, -42.630363, 36.70397));
    }

    #[test]
    fn ring_buffer_test() {
        let r = ring_buffer::exec(100, 7);
        let expected:Vec<i32> = (93..100).collect();
        assert_eq!(r.len(), 7);
        r.into_iter().for_each(|v|assert!(expected.contains(&v)));
    }

    #[test]
    fn runlength_test1() {
        let s = "AAAAAAAAAAAA".to_string();
        let rez = runlength::exec(s);
        assert_eq!(rez, "9A3A".to_string());
    }

    #[test]
    fn runlength_test2() {
        let s = "AABBBCDD".to_string();
        let rez = runlength::exec(s);
        assert_eq!(rez, "2A3B1C2D".to_string());
    }

    #[test]
    fn same_bst_true_case_test() {
        let a = vec![10, 15, 8, 12, 94, 81, 5, 2, 11];
        let b = vec![10, 8, 5, 15, 2, 12, 11, 94, 81];
        assert!(same_bst::exec(&a, &b));
    }


    #[test]
    fn same_bst_false_case_test() {
        let a = vec![10, 15, 8, 12, 94, 81, 5, 2, 1];
        let b = vec![10, 8, 5, 15, 2, 12, 11, 94, 81];
        assert_ne!(same_bst::exec(&a, &b), true);
    }

    #[test]
    fn shapearea_test() {
        assert_eq!(shapearea::exec(1), 1);
        assert_eq!(shapearea::exec(2), 5);
        assert_eq!(shapearea::exec(3), 13);
        assert_eq!(shapearea::exec(7000), 97986001);
    }

    #[test]
    fn shorten_path_absolute_path_case() {
        let path = "/foo/../test/../test/../foo//bar/./baz";
        let expected = "/foo/bar/baz".to_string();
        assert_eq!(shorten_path::exec(path), expected);
    }

    #[test]
    fn shorten_path_relative_path_case() {
        let path = "../tako/../test/../test/./baz";
        let expected = "test/baz".to_string();
        assert_eq!(shorten_path::exec(path), expected);
    }

    #[test]
    fn sigma_k_test() {
        let k = 7;
        let sigma_k = sigma_k::exec(k);
        let sum: i32 = (1..=k).sum();
        assert_eq!(sigma_k, sum);
    }

    #[test]
    fn single_cycle_check_test() {
        assert_eq!(single_cycle_check::exec(&mut [2, 3, 1, -4, -4, 2]), true);
        assert_eq!(single_cycle_check::exec(&mut [2, 6, 1, -4, -4, 4]), false);
    }

    #[test]
    fn single_stroke_test() {
        let mut matrix:Array2<i32> = arr2(&[
            [0, 1, 0, 1],
            [1, 0, 1, 2],
            [0, 1, 0, 1],
            [1, 2, 1, 0],
        ]);
        let num_edges = 6;
        let num_nodes = 4;
        let expected:Vec<Vec<i32>> = vec![
            vec![0, 1, 2, 3, 1, 3],
            vec![0, 1, 3, 1, 2, 3],
            vec![0, 1, 3, 2, 1, 3],
            vec![0, 3, 1, 2, 3, 1],
            vec![0, 3, 1, 3, 2, 1],
            vec![0, 3, 2, 1, 3, 1]
        ];
        let rez = single_stroke::exec(&mut matrix, 0, num_nodes, num_edges);
        for rec in rez {
            assert!(expected.contains(&rec));
        }
    }

    #[test]
    fn singly_linked_list_copy_test() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let mut list = build_singly_linkedlist(&v);
        let mut orig_trace:Vec<i32> = Vec::new();
        list.trav_from(&mut orig_trace);
        let copied = singly_linked_list_copy::exec(&mut list);
        let mut copied_trace:Vec<i32> = Vec::new();
        copied.trav_from(&mut copied_trace);
        assert_eq!(orig_trace, copied_trace);
    }

    #[test]
    fn singly_linked_list_reverse_test() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let mut list = build_singly_linkedlist(&v);
        let reversed = singly_linked_list_reverse::exec(&mut list);
        let mut reversed_trace:Vec<i32> = Vec::new();
        let expected:Vec<i32> = vec![6, 5, 4, 3, 2, 1];
        reversed.trav_from(&mut reversed_trace);
        assert_eq!(reversed_trace, expected);
    }

    #[test]
    fn smallest_difference_test() {
        let a = [-1, 5, 10, 20, 28, 3];
        let b = [26, 134, 135, 15, 17];
        let rez = smallest_difference::exec(&a, &b);
        assert_eq!(rez, (28, 26));
    }

    #[test]
    fn smallest_positive_good_case() {
        let n = 12;
        assert_eq!(smallest_positive_product::exec(n), 26);
    }

    #[test]
    fn smallest_positive_bad_case() {
        let n = 19;
        assert_eq!(smallest_positive_product::exec(n), -1);
    }

    #[test]
    fn smallest_sum_test() {
        assert_eq!(smallest_sum::exec(&[2, 4, 7]), 4);
        assert_eq!(smallest_sum::exec(&[2, 3]), 2);
    }

    #[test]
    fn sprite_index_test() {
        assert_eq!(sprite_index::exec(0, 40, 4), 0);
        assert_eq!(sprite_index::exec(8, 40, 4), 0);
        assert_eq!(sprite_index::exec(12, 40, 4), 1);
        assert_eq!(sprite_index::exec(23, 40, 4), 2);
        assert_eq!(sprite_index::exec(39, 40, 4), 3);
        assert_eq!(sprite_index::exec(40, 40, 4), 0);
    }

    #[test]
    fn softmax_test() {
        let x = vec![1.6, -2.3, 0.2, 3.4, -1.7, 0.5];
        let expected = vec![0.13, 0.003, 0.032, 0.787, 0.005, 0.043];
        assert_eq!(softmax::exec(x), expected);
    }

    #[test]
    fn softsign_test() {
        assert_eq!(softsign::exec(7.4), 0.8809524);
        assert_eq!(softsign::exec(0.7), 0.41176468);
        assert_eq!(softsign::exec(0.), 0.);
        assert_eq!(softsign::exec(2.2), 0.6875);
        assert_eq!(softsign::exec(-0.4), -0.2857143);
        assert_eq!(softsign::exec(-1.9), -0.6551724);
    }

    #[test]
    fn sort_by_height_test() {
        let mut v = vec![-1, 150, 190, 170, -1, -1, 160, 180];
        let expected = vec![-1, 150, 160, 170, -1, -1, 180, 190];
        sort_by_height::exec(&mut v);
        assert_eq!(v, expected);
    }

    #[test]
    fn sort_stack_test() {
        let mut v:Vec<i32> = vec![9, 16, 3, 8, 5, 3, 2, 1, 13, 10, 6];
        sort_stack::exec(&mut v);
        let expected = vec![1, 2, 3, 3, 5, 6, 8, 9, 10, 13, 16];
        assert_eq!(v, expected);
    }

    #[test]
    fn sorted_matrix_search_found_case() {
        let matrix:Array2<u32> = arr2(&[
            [1, 4, 7, 12, 15, 1000],
            [2, 5, 19, 31, 32, 1001],
            [3, 8, 24, 33, 35, 1002],
            [40, 41, 42, 44, 45, 1003],
            [99, 100, 103, 106, 128, 1004],
        ]);
        let rez = sorted_matrix_search::exec(&matrix, 44);
        assert_eq!(rez, (3, 3));
    }

    #[test]
    fn sorted_matrix_search_not_found_case() {
        let matrix:Array2<u32> = arr2(&[
            [1, 4, 7, 12, 15, 1000],
            [2, 5, 19, 31, 32, 1001],
            [3, 8, 24, 33, 35, 1002],
            [40, 41, 42, 44, 45, 1003],
            [99, 100, 103, 106, 128, 1004],
        ]);
        let rez = sorted_matrix_search::exec(&matrix, 43);
        assert_eq!(rez, (-1, -1));
    }

    #[test]
    fn stack_copy_only_pop_test() {
        let mut v = vec![1, 2, 3, 4, 5];
        let expected = v.clone();
        let mut r:Vec<i32> = Vec::new();
        stack_copy_only_pop::exec(&mut v, &mut r);
        assert_eq!(expected, r);
    }

    #[test]
    fn stdev_test() {
        let v = &[71_f32, 80_f32, 89_f32];
        let stdev = stdev::exec(v);
        assert!(float_cmp::approx_eq!(f32, stdev, 7.3484693_f32, ulps = 2));
    }

    #[test]
    fn storage_projection_for_sdhd_test() {
        let hd_num = 42;
        let sd_num = 27;
        let expected_total_per_h = 1110;
        let expected_total_per_s = 264;
        assert_eq!(storage_projection_for_sdhd::exec(sd_num, hd_num), (expected_total_per_h, expected_total_per_s));
    }

    #[test]
    fn strange_bank_ok_test() {
        let n = 7_u32;
        let denoms = &[1, 5, 10];
        let rez = strange_bank::exec(n, denoms);
        assert_eq!(rez, 3);
    }

    #[test]
    fn strange_bank_nochange_made_test() {
        let n = 9_u32;
        let denoms = &[4, 6, 10, 50];
        let rez = strange_bank::exec(n, denoms);
        assert_eq!(rez, -1);
    }

    #[test]
    fn string_construction_test1() {
        let a = "abc";
        let b = "abccba";
        assert_eq!(string_construction::exec(a, b), 2);
    }

    #[test]
    fn string_construction_test2() {
        let a = "abc";
        let b = "abba";
        assert_eq!(string_construction::exec(a, b), 0);
    }

    #[test]
    fn string_pattern_test_simplecase() {
        let input = "21";
        let expected = "1211";
        assert_eq!(string_pattern::exec(input).as_str(), expected);
    }

    #[test]
    fn string_pattern_test_complexcase() {
        let input = "33372211115";
        let expected = "3317224115";
        assert_eq!(string_pattern::exec(input).as_str(), expected);
    }

    #[test]
    fn subarray_sort_tobe_sorted_case() {
        let a = vec![1, 2, 4, 7, 10, 11, 7, 12, 6, 7, 16, 18, 19];
        assert_eq!(subarray_sort::exec(&a).unwrap(), (3, 9));
    }


    #[test]
    fn subarray_already_sorted_case() {
        let a = vec![1, 2, 4, 6, 7, 10, 11, 12, 16, 18, 19];
        match subarray_sort::exec(&a) {
            None => (),
            Some(_) => assert!(false),
        }
    }

    #[test]
    fn suffix_trie_test() {
        let rez = suffix_trie::exec(&["neko", "nekoya", "takoya", "tako", "pokoya", "nyanko", "nek"], "neko");
        assert_eq!(rez, vec!["neko".to_string()]);
    }

    #[test]
    fn sum_of_arithmatic_progression_test() {
        assert_eq!(sum_of_arithmatic_progression::exec(4, 5, 4, 3), 34);
        assert_eq!(sum_of_arithmatic_progression::exec(5, 5, 4, 3), 50);
    }

    #[test]
    fn sum_of_squared_deviations_test() {
        let v:Vec<f32> = vec![4.0, 5.0, 6.0, 7.0, 9.0, 12.0, 15.0, 16.0];
        let r = sum_of_squared_deviation::exec(&v);
        approx_eq!(f32, r, 147.5_f32);
    }


    #[test]
    fn sunset_view_test() {
        let bldgs1 = [3, 5, 4, 4, 3, 1, 3, 2];
        let bldgs2 = [2, 4, 4, 5, 1, 2, 8, 7, 10, 4];
        let expected1:Vec<usize> = vec![1, 3, 6, 7];
        let expected2 = vec![0, 1, 3, 6, 8];
        let rez1:Vec<usize> = sunset_view::exec(&bldgs1, "east").into_iter().collect();
        let rez2:Vec<usize> = sunset_view::exec(&bldgs2, "west").into_iter().collect();
        assert_eq!(rez1, expected1);
        assert_eq!(rez2, expected2);
    }

    #[test]
    fn tandem_bike_test() {
        assert_eq!(tandem_bike::exec(&mut [5, 5, 3, 9, 2], &mut [1, 6, 7, 2, 1], true), 32);
        assert_eq!(tandem_bike::exec(&mut [5, 5, 3, 9, 2], &mut [1, 6, 7, 2, 1], false), 25);
    }

    #[test]
    fn tandem_repeat_case1() {
        let c = "CatCat".to_string();
        assert!(tandem_repeat::exec(c));
    }

    #[test]
    fn task_assignment_test() {
        let k = 3;
        let t = vec![1, 3, 5, 3, 1, 4];
        let expected1 = vec![(0, 2), (2, 0), (2, 4), (4, 2)];
        let expected2 = vec![(4, 5), (5, 4), (0, 5), (5, 0)];
        let expected3 = vec![(1, 3), (3, 1)];
        let expected = vec![expected1, expected2, expected3];
        let rez = task_assignment::exec(k, t);
        for (i, r) in rez.iter().enumerate() {
            println!("{:?} in {:?}?", r, expected[i]);
            assert!(expected[i].contains(r));
        }
    }

    #[test]
    fn dec_to_bin_test1() {
        let n = 23;
        let b = dectobin::exec(n);
        assert_eq!(b, "10111".to_string());
    }

    #[test]
    fn three_sum_test() {
        let mut a = vec![12, 3, 1, 2, -6, 5, -8, 6];
        let target = 0;
        let rez = three_sum::exec(&mut a, target);
        assert_eq!(rez.len(), 3);
        assert_eq!(rez[0], vec![-8, 2, 6]);
        assert_eq!(rez[1], vec![-8, 3, 5]);
        assert_eq!(rez[2], vec![-6, 1, 5]);
    }

    #[test]
    fn total_depth_sum_of_tree_test() {

        let node = build_tree(&vec![50, 20, 100, 10, 30, 80, 150, 5, 15]);
        assert_eq!(total_depth_sum_of_tree::exec(&node), 16);
    }

    #[test]
    fn total_sum_with_n_num_test() {
        let a = &[2, 3, 4, 5];
        let target = 14;
        let k = 4;
        assert!(total_sum_with_n_num::exec(a, target, k));
    }

    #[test]
    fn diagonal_is_diag_yes() {
        let a = Point::new(1, 5);
        let b = Point::new(9, 13);
        println!(
            "a:{},b:{} must be diagonal - result is {}",
            a,
            b,
            diagonal::exec(&a, &b)
        );
        assert!(diagonal::exec(&a, &b));
    }


    #[test]
    fn diagonal_is_diag_no() {
        let a = Point::new(3, 5);
        let b = Point::new(8, 5);
        println!(
            "a:{},b:{} must be diagonal - result is {}",
            a,
            b,
            diagonal::exec(&a, &b)
        );
        assert!(diagonal::exec(&a, &b) == false);
    }

    #[test]
    fn document_build_test() {
        let s = "Todd told Tom to trot to the timber".to_string();
        let expected: Vec<char> = vec![' ', 'T', 'b', 'd', 'e', 'h', 'i', 'l', 'm', 'o', 'r', 't'];
        assert_eq!(document_build::exec(s), expected);
    }

    #[test]
    fn product_array_sort_test() {
        let v = vec![-11, -6, 0, 5, 8, 10];
        let w = product_array_sort::exec(&v);
        assert_eq!(w, vec![0, 25, 36, 64, 100, 121]);
    }

    #[test]
    fn rightmost_diffbit_case1() {
        let m = 13;
        let n = 11;
        assert_eq!(2, rightmost_diffbit::exec(m, n));
    }

    #[test]
    fn rightmost_diffbit_case2() {
        let m = 7;
        let n = 23;
        assert_eq!(16, rightmost_diffbit::exec(m, n));
    }

    #[test]
    fn rightmost_samebit_case1() {
        let m = 10;
        let n = 11;
        assert_eq!(2, rightmost_samebit::exec(m, n));
    }

    #[test]
    fn sturge_formula_test() {
        let n = 9072.;
        let k = sturge_formula::exec(n);
        assert_eq!(k, 14);
    }

    #[test]
    fn sum_of_consecutive_test() {
        let empty_result:Vec<Vec<i32>> = Vec::new();
        assert_eq!(sum_of_consecutive_integers::exec(9), vec![vec![2, 3, 4], vec![4, 5]]);
        assert_eq!(sum_of_consecutive_integers::exec(8), empty_result);
        assert_eq!(sum_of_consecutive_integers::exec(27), vec![vec![2, 3, 4, 5, 6, 7], vec![8, 9, 10], vec![13, 14]]);
        assert_eq!(sum_of_consecutive_integers::exec(25), vec![vec![3, 4, 5, 6, 7], vec![12, 13]]);
        assert_eq!(sum_of_consecutive_integers::exec(19), vec![vec![9, 10]]);
    }

    #[test]
    fn sum_of_integers_test() {
        let mut expected:HashSet<Vec<i32>> = HashSet::new();
        expected.insert(vec![9]);
        expected.insert(vec![2, 3, 4]);
        expected.insert(vec![4, 5]);
        expected.insert(vec![2, 7]);
        expected.insert(vec![3, 6]);
        assert_eq!(sum_of_integers::exec(9), expected);
    }

    #[test]
    fn swap_sibling_test() {
        let mut v: Vec<i32> = (1..=6).collect();
        swap_sibling::exec(&mut v);
        let expected: Vec<i32> = vec![2, 1, 4, 3, 6, 5];
        assert_eq!(v, expected);
    }

    #[test]
    fn sudoku_test() {
        let mut m = arr2(&[
            [-1, 1, 5, 9, -1, 4, -1, -1, -1],
            [6, 8, 3, -1, -1, 2, -1, -1, 7],
            [-1, 9, -1, -1, 3, -1, -1, -1, -1],
            [1, -1, -1, -1, 4, -1, -1, -1, 3],
            [9, 4, -1, -1, 6, 7, -1, 2, -1],
            [-1, 7, 2, -1, 8, 1, 4, -1, 6],
            [-1, 6, 1, -1, -1, 3, 2, 5, -1],
            [-1, 3, -1, 4, 2, 6, -1, 7, 1],
            [7, -1, -1, 8, 1, -1, -1, 6, 9]
            ]);
        sudoku::exec(&mut m);
        let expected:Array2<i32> = arr2(&[
            [2, 1, 5, 9, 7, 4, 6, 3, 8],
            [6, 8, 3, 1, 5, 2, 9, 4, 7],
            [4, 9, 7, 6, 3, 8, 5, 1, 2],
            [1, 5, 6, 2, 4, 9, 7, 8, 3],
            [9, 4, 8, 3, 6, 7, 1, 2, 5],
            [3, 7, 2, 5, 8, 1, 4, 9, 6],
            [8, 6, 1, 7, 9, 3, 2, 5, 4],
            [5, 3, 9, 4, 2, 6, 8, 7, 1],
            [7, 2, 4, 8, 1, 5, 3, 6, 9]
        ]);
        assert_eq!(m.view(), expected.view());
    }

    #[test]
    fn tandem_repeat_case2() {
        let a = "ABA".to_string();
        let b = "cattac".to_string();
        assert!(!tandem_repeat::exec(a) && !tandem_repeat::exec(b));
    }

    #[test]
    fn three_number_sort_test() {
        let order = [0, 1, -1];
        let mut arr = [1, 0, 0, -1, -1, 0, 1, 1];
        let expected = [0, 0, 0, 1, 1, 1, -1, -1];
        three_number_sort::exec(&mut arr, &order);
        assert_eq!(arr, expected);
    }

    #[test]
    fn toggle_bit_test() {
        let n = 15;
        let k = 2;
        let rez1 = toggle_bit::exec(n, k);
        assert_eq!(rez1, 13);
        let rez2 = toggle_bit::exec(rez1, k);
        assert_eq!(rez2, n);
    }

    #[test]
    fn topological_sort() {
        let j = vec![1, 2, 3, 4];
        let mut deps:Vec<Vec<i32>> = Vec::new();
        deps.push(vec![1, 2]);
        deps.push(vec![1, 3]);
        deps.push(vec![3, 2]);
        deps.push(vec![4, 2]);
        deps.push(vec![4, 3]);
        let rez = topological_sort::exec(&j, &deps);
        assert!(rez.is_some());
        let r = rez.unwrap();
        assert_eq!(r, vec![4, 1, 3, 2]);
    }

    #[test]
    fn turn_commands_test() {
        let cmd = "LLARL";
        assert_eq!(turn_commands::exec(cmd), 3);
    }

    #[test]
    fn two_sum_case1() {
        let rez = two_sum::exec(vec![3, 5, -4, 8, 11, 1, -1, 6], 10);
        assert_eq!(rez, vec![-1, 11])
    }

    #[test]
    fn two_sum_case2() {
        let rez = two_sum::exec(vec![4, 6], 10);
        let rset: HashSet<i32> = HashSet::from_iter(rez);
        let expected: HashSet<i32> = HashSet::from_iter([4, 6]);
        assert_eq!(rset, expected);
    }

    #[test]
    fn two_sum_case3() {
        let rez = two_sum::exec(vec![-21, 301, 12, 4, 65, 56, 210, 356, 9, -47], 163);
        let rset: HashSet<i32> = HashSet::from_iter(rez);
        let expected: HashSet<i32> = HashSet::from_iter([210, -47]);
        assert_eq!(rset, expected);
    }

    #[test]
    fn sel_sort_test() {
        let mut n = vec![1, 9, 4, 10, -3, 9, 15, 2];
        selection_sort::exec(&mut n);
        assert_eq!(n, vec![-3, 1, 2, 4, 9, 9, 10, 15]);
    }

    #[test]
    fn unconstructiable_number_test() {
        let mut v = [5, 7, 1, 1, 2, 3, 22];
        assert_eq!(unconstructiable_number::exec(&mut v), 20);
    }

    #[test]
    fn valid_parentheses_test() {
        assert_eq!(valid_parentheses::exec("{[[((()))]]}"), true);
        assert_eq!(valid_parentheses::exec("{[neko]}"), true);
        assert_eq!(valid_parentheses::exec("(([))"), false);
        assert_eq!(valid_parentheses::exec("[[]"), false);
        assert_eq!(valid_parentheses::exec("[]]"), false);
    }

    #[test]
    fn valid_starting_city_all_no_issue_test() {
        let dist = vec![5, 25, 15, 10, 15];
        let fuel = vec![1, 2, 2, 1, 0];
        let mpg = 10;
        assert_eq!(valid_starting_city::exec(dist, fuel, mpg), 0);
    }

    #[test]
    fn valid_starting_city_stamble_at_middle_test() {
        let dist = vec![5, 25, 15, 10, 15];
        let fuel = vec![1, 2, 1, 0, 3];
        let mpg = 10;
        assert_eq!(valid_starting_city::exec(dist, fuel, mpg), 4);
    }

    #[test]
    fn validate_three_nodes_test() {
        let n = build_tree(&vec![5, 2, 1, 0, 4, 3, 7, 6, 8]);
        let one = 5;
        let two = 2;
        let three = 3;
        assert!(validate_three_nodes::exec(n, one, two, three));
    }

    #[test]
    fn variance_test() {
        let v = &[71_f32, 80_f32, 89_f32];
        assert_eq!(variance::exec(v), 54_f32);
    }

    #[test]
    fn waterarea_test() {
        let a: Vec<u32> = vec![0, 8, 0, 0, 5, 0, 0, 10, 0, 0, 1, 1, 0, 3];
        let r = waterarea::exec(a);
        assert_eq!(r, 48);
    }

    #[test]
    fn xor_shift_test() {
        let v = Vec::from_iter(std::iter::repeat_with(||xor_shift::exec(10000000)).take(5));
        let s:std::collections::HashSet<u128> = v.into_iter().collect();
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn yuv_to_rgb_test() {
        let (y, u, v) = (203.7705, -42.630363, 36.70397);
        assert_eq!(yuv_to_rgb::exec(y, u, v), (255, 192, 128));
    }

    #[test]
    fn zigzag_traversal_test() {
        let m:Array2<i32> = arr2(&[
            [1, 3, 4, 10],
            [2, 5, 9, 11],
            [6, 8, 12, 15],
            [7, 13, 14, 16],
        ]);
        let rez = zigzag_traversal::exec(&m);
        let expected:Vec<i32> = (1..=16).collect();
        assert_eq!(rez, expected);

    }

    #[test]
    fn zscore_test() {
        let mut v = [7_f32, 8_f32, 8_f32, 7.5_f32, 9_f32];
        let zscores = z_score::exec(&mut v);
        let expected = [-1.5075567, -0.75377834, 0.0, 0.0, 1.5075567];
        assert!(zscores.iter().zip(expected.iter()).map(|(z, e)|approx_eq!(f32, *z, *e)).all(|r|r));
    }

}

pub enum Algo {
    AddTwoIntWithoutCarry,
    AllLongestStrings,
    AlmostIncreasingSeq,
    AlphabetSubseq,
    AppearTwice,
    Applebox,
    ArithmaticProgression,
    ArrayIsSmooth,
    ArrayProductSum,
    Average,
    BinarySearch,
    Bincount,
    BinToDec,
    BubbleSort,
    BuildBTreeFromInorderPreorder,
    BuildClosureTag,
    BuildHeap,
    BuildIPAddr,
    BuildPalindrome,
    CamelCase,
    CanIncreaseWithRoundness,
    CanMakeSumAWithKNum,
    Capitalize,
    CelciusToFahrenheit,
    Century,
    CaesarCrypt,
    CharCount,
    CharsAppearingTwice,
    CharsPermList,
    CharsToSortedDigits,
    ChristmasTree,
    ClampValue,
    Classmark,
    CleanKthBit,
    CodeBreaker,
    Combination,
    CompareAndSwap,
    ConstructMinHeightBST,
    ConstructSquare,
    ContrastCheck,
    CoolNumPair,
    CrossEntropy,
    CumsumExceptLast,
    CyclicChars,
    DecodeReversePoland,
    DecToBin,
    DegToRad,
    DFS,
    Diagonal,
    DifferentSquares,
    DifferentValuePairs,
    DNSWaterTorture,
    DocumentBuild,
    EvalTicTacToe,
    EvenNumSum,
    Euclidean,
    Factorial,
    FahrenheitToCelcius,
    Fibonacci,
    FileNaming,
    FindClosestValue,
    FindMaxValThread,
    FindSuccessor,
    FirstDupeValue,
    FizzBuzz,
    FSMMealy,
    FSMMoor,
    GeometricProgression,
    GeometricalMean,
    GroupAnagrams,
    GroupByClassRange,
    GroupByKey,
    GuardTechnique,
    HanoiTower,
    HarmonicMean,
    HelonFormula,
    HumanAndCatlegs,
    ImageStride,
    InOrderTraversal,
    InsertionSort,
    InvertBTree,
    IPv4,
    IsBST,
    IslandSize,
    IsMacAddr,
    IsPalindrome,
    IsTwoArraySimilar,
    JumpHeightInFrames,
    Kadane,
    Knapsack,
    KthElementInTree,
    LagrangeInterpolationPolynominal,
    LargestAdjacentProduct,
    LargestNumber,
    LCS,
    LeastGreatest,
    LeastDataEviction,
    LeastFactorial,
    LeastLSD,
    LengthOfLinkedList,
    LevenShteinDistance,
    LinkedList,
    LocByLineAngle,
    LongestPelindromicSubstring,
    LongestSubstringWithoutDupe,
    LowerUpperHinge,
    LRValueSumCombo,
    L0Norm,
    L1Norm,
    MaxNumByDelOneDigit,
    MaxPathSum,
    MaxSiblingProduct,
    MaxSubSetSum,
    MaxWithLessDigit,
    Mean,
    Merge2Lists,
    MergeSortedLinkedList,
    MinBreakdownSum,
    Minesweeper,
    MinimumWaitingGame,
    MinMaxStack,
    MinPassesMatrixUpdate,
    MinReward,
    Mode,
    MonotonicArray,
    MostFrequentDigitSum,
    NextGreaterElement,
    NegaPosiInversion,
    NodeDistanceK,
    NSteps,
    NumberGrouping,
    NumOfClans,
    NumOfPaths,
    ObtainIncreasingSeq,
    OctalPermission,
    OctalToDec,
    OppositePosInCircle,
    OverlappingIntervals,
    PascalTriangle,
    PartialSumCount,
    PeakInArray,
    Permutation,
    PermutationMatrix,
    PhoneMnemonic,
    PostOrderTraversal,
    PowerSet,
    PrefixSums,
    PreOrderTraversal,
    ProductArraySort,
    PseudoRand,
    Quarter,
    QuickSort,
    RadixSort,
    RadToDeg,
    RandomPerm,
    Ranking,
    ReconstructBSTFromPreorder,
    RemoveIsland,
    RemoveNthFromEnd,
    RepeatProduct,
    ReplaceMidValue,
    RequestPerSec,
    ReverseInParenthiesis,
    ReversePoland,
    ReverseStack,
    ReverseWords,
    RgbToBgr,
    RgbToYuv,
    RightMostDiffBit,
    RightMostSameBit,
    RingBuffer,
    RunLength,
    SameBST,
    ShapeArea,
    SelectionSort,
    ShortenPath,
    SigmaK,
    SingleCycleCheck,
    SingleStroke,
    SinglyLinkedListCopy,
    SinglyLinkedListReverse,
    SmallestDifference,
    SmallestPositiveProduct,
    SmallestSum,
    SpriteIndex,
    Softmax,
    Softsign,
    SortByHeight,
    SortStack,
    SortedMatrixSearch,
    StackCopyOnlyPop,
    Stdev,
    StorageProjectionForSDHD,
    StrangeBank,
    StringConstruction,
    StringPattern,
    SturgeFormula,
    SubArraySort,
    Sudoku,
    SuffixTrue,
    SumOfArithmaticProgression,
    SumOfConsective,
    SumOfIntegers,
    SumOfSquaredDeviations,
    SunsetView,
    SwapSibling,
    TandemBike,
    TandemRepeat,
    TaskAssignment,
    ThreeNumberSort,
    ThreeSum,
    ToggleBit,
    TopologicalSort,
    TotalDepthSumOfTree,
    TotalSumWithNNums,
    TurnCommands,
    TwoSum,
    UnconstructiableNumber,
    ValidateThreeNodes,
    ValidParentheses,
    ValidStartingCity,
    Variance,
    WaterArea,
    XOrShift,
    YuvToRgb,
    zigzagTraversal,
    ZScore,
}

impl Algo {
    pub fn from_str(algo_str: &str) -> Self {
        match algo_str {
            s if s.to_lowercase() == "add_two_int_without_carry" => Algo::AddTwoIntWithoutCarry,
            s if s.to_lowercase() == "all_longest_strings" => Algo::AllLongestStrings,
            s if s.to_lowercase() == "almost_increasing_seq" => Algo::AlmostIncreasingSeq,
            s if s.to_lowercase() == "alphabet_subseq" => Algo::AlphabetSubseq,
            s if s.to_lowercase() == "appear_twice" => Algo::AppearTwice,
            s if s.to_lowercase() == "applebox" => Algo::Applebox,
            s if s.to_lowercase() == "arithmatic_progression" => Algo::ArithmaticProgression,
            s if s.to_lowercase() == "array_is_smooth" => Algo::ArrayIsSmooth,
            s if s.to_lowercase() == "array_product_sum" => Algo::ArrayProductSum,
            s if s.to_lowercase() == "average" => Algo::Average,
            s if s.to_lowercase() == "binary_search" => Algo::BinarySearch,
            s if s.to_lowercase() == "bincount" => Algo::Bincount,
            s if s.to_lowercase() == "bintodec" => Algo::BinToDec,
            s if s.to_lowercase() == "bubble_sort" => Algo::BubbleSort,
            s if s.to_lowercase() == "build_bt_from_preorder_inorder" => Algo::BuildBTreeFromInorderPreorder,
            s if s.to_lowercase() == "build_closure_tag" => Algo::BuildClosureTag,
            s if s.to_lowercase() == "build_heap" => Algo::BuildHeap,
            s if s.to_lowercase() == "build_ip_addr" => Algo::BuildIPAddr,
            s if s.to_lowercase() == "build_palindrome" => Algo::BuildPalindrome,
            s if s.to_lowercase() == "caesar_crypt" => Algo::CaesarCrypt,
            s if s.to_lowercase() == "camelcase" => Algo::CamelCase,
            s if s.to_lowercase() == "can_increase_with_roundness" => Algo::CanIncreaseWithRoundness,
            s if s.to_lowercase() == "can_make_sum_a_with_k_num" => Algo::CanMakeSumAWithKNum,
            s if s.to_lowercase() == "capitalize" => Algo::Capitalize,
            s if s.to_lowercase() == "celcius_to_fahrenheit" => Algo::CelciusToFahrenheit,
            s if s.to_lowercase() == "century" => Algo::Century,
            s if s.to_lowercase() == "char_count" => Algo::CharCount,
            s if s.to_lowercase() == "chars_appearing_twice" => Algo::CharsAppearingTwice,
            s if s.to_lowercase() == "chars_perm_list" => Algo::CharsPermList,
            s if s.to_lowercase() == "chars_to_sorted_digits" => Algo::CharsToSortedDigits,
            s if s.to_lowercase() == "christmas_tree" => Algo::ChristmasTree,
            s if s.to_lowercase() == "clamp_value" => Algo::ClampValue,
            s if s.to_lowercase() == "classmark" => Algo::Classmark,
            s if s.to_lowercase() == "clean_kth_bit" => Algo::CleanKthBit,
            s if s.to_lowercase() == "code_breaker" => Algo::CodeBreaker,
            s if s.to_lowercase() == "combination" => Algo::Combination,
            s if s.to_lowercase() == "compare_and_swap" => Algo::CompareAndSwap,
            s if s.to_lowercase() == "construct_min_height_bst" => Algo::ConstructMinHeightBST,
            s if s.to_lowercase() == "construct_square" => Algo::ConstructSquare,
            s if s.to_lowercase() == "contrast_check" => Algo::ContrastCheck,
            s if s.to_lowercase() == "cool_num_pair" => Algo::CoolNumPair,
            s if s.to_lowercase() == "cross_entropy" => Algo::CrossEntropy,
            s if s.to_lowercase() == "cumsum_except_last" => Algo::CumsumExceptLast,
            s if s.to_lowercase() == "cyclic_chars" => Algo::CyclicChars,
            s if s.to_lowercase() == "decode_reverse_poland" => Algo::DecodeReversePoland,
            s if s.to_lowercase() == "dectobin" => Algo::DecToBin,
            s if s.to_lowercase() == "deg_to_rad" => Algo::DegToRad,
            s if s.to_lowercase() == "dfs" => Algo::DFS,
            s if s.to_lowercase() == "diagonal" => Algo::Diagonal,
            s if s.to_lowercase() == "different_squares" => Algo::DifferentSquares,
            s if s.to_lowercase() == "different_value_pairs" => Algo::DifferentValuePairs,
            s if s.to_lowercase() == "dns_water_torture" => Algo::DNSWaterTorture,
            s if s.to_lowercase() == "document_build" => Algo::DocumentBuild,
            s if s.to_lowercase() == "eval_tictactoe" => Algo::EvalTicTacToe,
            s if s.to_lowercase() == "even_num_sum" => Algo::EvenNumSum,
            s if s.to_lowercase() == "euclidean" => Algo::Euclidean,
            s if s.to_lowercase() == "factorial" => Algo::Factorial,
            s if s.to_lowercase() == "fahrenheit_to_celcius" => Algo::FahrenheitToCelcius,
            s if s.to_lowercase() == "fibonacci" => Algo::Fibonacci,
            s if s.to_lowercase() == "file_naming" => Algo::FileNaming,
            s if s.to_lowercase() == "find_closest_value" => Algo::FindClosestValue,
            s if s.to_lowercase() == "find_max_val_thread" => Algo::FindMaxValThread,
            s if s.to_lowercase() == "find_successor" => Algo::FindSuccessor,
            s if s.to_lowercase() == "first_dupe_value" => Algo::FirstDupeValue,
            s if s.to_lowercase() == "fizzbuzz" => Algo::FizzBuzz,
            s if s.to_lowercase() == "fsm_mealy" => Algo::FSMMealy,
            s if s.to_lowercase() == "fsm_moor" => Algo::FSMMoor,
            s if s.to_lowercase() == "geometric_progression" => Algo::GeometricProgression,
            s if s.to_lowercase() == "geometrical_mean" => Algo::GeometricalMean,
            s if s.to_lowercase() == "group_anagrams" => Algo::GroupAnagrams,
            s if s.to_lowercase() == "group_by_class_range" => Algo::GroupByClassRange,
            s if s.to_lowercase() == "group_by_key" => Algo::GroupByKey,
            s if s.to_lowercase() == "guard_technique" => Algo::GuardTechnique,
            s if s.to_lowercase() == "hanoi_tower" => Algo::HanoiTower,
            s if s.to_lowercase() == "harmonic_mean" => Algo::HarmonicMean,
            s if s.to_lowercase() == "helon_formula" => Algo::HelonFormula,
            s if s.to_lowercase() == "human_cat_legs" => Algo::HumanAndCatlegs,
            s if s.to_lowercase() == "image_stride" => Algo::ImageStride,
            s if s.to_lowercase() == "inorder_traversal" => Algo::InOrderTraversal,
            s if s.to_lowercase() == "invert_btree" => Algo::InvertBTree,
            s if s.to_lowercase() == "insertion_sort" => Algo::InsertionSort,
            s if s.to_lowercase() == "ipv4" => Algo::IPv4,
            s if s.to_lowercase() == "is_bst" => Algo::IsBST,
            s if s.to_lowercase() == "is_palindrome" => Algo::IsPalindrome,
            s if s.to_lowercase() == "is_mac_addr" => Algo::IsMacAddr,
            s if s.to_lowercase() == "is_two_array_similar" => Algo::IsTwoArraySimilar,
            s if s.to_lowercase() == "island_size" => Algo::IslandSize,
            s if s.to_lowercase() == "jump_height_in_frames" => Algo::JumpHeightInFrames,
            s if s.to_lowercase() == "kadane" => Algo::Kadane,
            s if s.to_lowercase() == "knapsack" => Algo::Knapsack,
            s if s.to_lowercase() == "kth_element_in_tree" => Algo::KthElementInTree,
            s if s.to_lowercase() == "l_r_value_sum_combo" => Algo::LRValueSumCombo,
            s if s.to_lowercase() == "lagrange_interpolation_polynominal" => Algo::LagrangeInterpolationPolynominal,
            s if s.to_lowercase() == "largest_adjacent_product" => Algo::LargestAdjacentProduct,
            s if s.to_lowercase() == "largest_number" => Algo::LargestNumber,
            s if s.to_lowercase() == "lcs" => Algo::LCS,
            s if s.to_lowercase() == "least_data_eviction" => Algo::LeastDataEviction,
            s if s.to_lowercase() == "least_factorial" => Algo::LeastFactorial,
            s if s.to_lowercase() == "least_greatest" => Algo::LeastGreatest,
            s if s.to_lowercase() == "least_lsd" => Algo::LeastLSD,
            s if s.to_lowercase() == "length_of_linkedlist" => Algo::LengthOfLinkedList,
            s if s.to_lowercase() == "levenshtein_distance" => Algo::LevenShteinDistance,
            s if s.to_lowercase() == "linked_list" => Algo::LinkedList,
            s if s.to_lowercase() == "loc_by_line_angle" => Algo::LocByLineAngle,
            s if s.to_lowercase() == "longest_pelindromic_substring" => Algo::LongestPelindromicSubstring,
            s if s.to_lowercase() == "longest_substring_without_dupe" => Algo::LongestSubstringWithoutDupe,
            s if s.to_lowercase() == "lower_upper_hinge" => Algo::LowerUpperHinge,
            s if s.to_lowercase() == "l0_norm" => Algo::L0Norm,
            s if s.to_lowercase() == "l1_norm" => Algo::L1Norm,
            s if s.to_lowercase() == "max_path_sum" => Algo::MaxPathSum,
            s if s.to_lowercase() == "max_sibling_product" => Algo::MaxSiblingProduct,
            s if s.to_lowercase() == "max_subset_sum" => Algo::MaxSubSetSum,
            s if s.to_lowercase() == "max_with_lessdigit" => Algo::MaxWithLessDigit,
            s if s.to_lowercase() == "maxnum_by_del_one_digit" => Algo::MaxNumByDelOneDigit,
            s if s.to_lowercase() == "mean" => Algo::Mean,
            s if s.to_lowercase() == "merge_2_lists" => Algo::Merge2Lists,
            s if s.to_lowercase() == "merge_sorted_linkedlist" => Algo::MergeSortedLinkedList,
            s if s.to_lowercase() == "min_breakdown_sum" => Algo::MinBreakdownSum,
            s if s.to_lowercase() == "min_passes_matrix_update" => Algo::MinPassesMatrixUpdate,
            s if s.to_lowercase() == "min_reward" => Algo::MinReward,
            s if s.to_lowercase() == "minesweeper" => Algo::Minesweeper,
            s if s.to_lowercase() == "minimum_waiting_game" => Algo::MinimumWaitingGame,
            s if s.to_lowercase() == "minmax_stack" => Algo::MinMaxStack,
            s if s.to_lowercase() == "mode" => Algo::Mode,
            s if s.to_lowercase() == "monotonic_array" => Algo::MonotonicArray,
            s if s.to_lowercase() == "most_frequent_digit_sum" => Algo::MostFrequentDigitSum,
            s if s.to_lowercase() == "n_steps" => Algo::NSteps,
            s if s.to_lowercase() == "nega_posi_inversion" => Algo::NegaPosiInversion,
            s if s.to_lowercase() == "next_greater_element" => Algo::NextGreaterElement,
            s if s.to_lowercase() == "node_distance_k" => Algo::NodeDistanceK,
            s if s.to_lowercase() == "num_of_clans" => Algo::NumOfClans,
            s if s.to_lowercase() == "num_of_paths" => Algo::NumOfPaths,
            s if s.to_lowercase() == "number_grouping" => Algo::NumberGrouping,
            s if s.to_lowercase() == "obtain_increasing_seq" => Algo::ObtainIncreasingSeq,
            s if s.to_lowercase() == "octal_permission" => Algo::OctalPermission,
            s if s.to_lowercase() == "octal_to_dec" => Algo::OctalToDec,
            s if s.to_lowercase() == "opposite_loc_in_circle" => Algo::OppositePosInCircle,
            s if s.to_lowercase() == "overlapping_intervals" => Algo::OverlappingIntervals,
            s if s.to_lowercase() == "pascal_triangle" => Algo::PascalTriangle,
            s if s.to_lowercase() == "partial_sum_count" => Algo::PartialSumCount,
            s if s.to_lowercase() == "peak_in_array" => Algo::PeakInArray,
            s if s.to_lowercase() == "permutation" => Algo::Permutation,
            s if s.to_lowercase() == "permutation_matrix" => Algo::PermutationMatrix,
            s if s.to_lowercase() == "phone_mnemonic" => Algo::PhoneMnemonic,
            s if s.to_lowercase() == "postorder_traversal" => Algo::PostOrderTraversal,
            s if s.to_lowercase() == "powerset" => Algo::PowerSet,
            s if s.to_lowercase() == "prefix_sums" => Algo::PrefixSums,
            s if s.to_lowercase() == "preorder_traversal" => Algo::PreOrderTraversal,
            s if s.to_lowercase() == "product_array_sort" => Algo::ProductArraySort,
            s if s.to_lowercase() == "pseudo_rand" => Algo::PseudoRand,
            s if s.to_lowercase() == "quarter" => Algo::Quarter,
            s if s.to_lowercase() == "quick_sort" => Algo::QuickSort,
            s if s.to_lowercase() == "rad_to_deg" => Algo::RadToDeg,
            s if s.to_lowercase() == "radix_sort" => Algo::RadixSort,
            s if s.to_lowercase() == "random_perm" => Algo::RandomPerm,
            s if s.to_lowercase() == "ranking" => Algo::Ranking,
            s if s.to_lowercase() == "reconstruct_bst_from_preorder" => Algo::ReconstructBSTFromPreorder,
            s if s.to_lowercase() == "remove_island" => Algo::RemoveIsland,
            s if s.to_lowercase() == "remove_nth_from_end" => Algo::RemoveNthFromEnd,
            s if s.to_lowercase() == "repeatproduct" => Algo::RepeatProduct,
            s if s.to_lowercase() == "replace_mid_value" => Algo::ReplaceMidValue,
            s if s.to_lowercase() == "request_per_sec" => Algo::RequestPerSec,
            s if s.to_lowercase() == "reverse_in_parenthiesis" => Algo::ReverseInParenthiesis,
            s if s.to_lowercase() == "reverse_poland" => Algo::ReversePoland,
            s if s.to_lowercase() == "reverse_stack" => Algo::ReverseStack,
            s if s.to_lowercase() == "reverse_words" => Algo::ReverseWords,
            s if s.to_lowercase() == "rgb_to_bgr" => Algo::RgbToBgr,
            s if s.to_lowercase() == "rgb_to_yuv" => Algo::RgbToYuv,
            s if s.to_lowercase() == "rightmost_diffbit" => Algo::RightMostDiffBit,
            s if s.to_lowercase() == "rightmost_samebit" => Algo::RightMostSameBit,
            s if s.to_lowercase() == "ring_buffer" => Algo::RingBuffer,
            s if s.to_lowercase() == "runlength" => Algo::RunLength,
            s if s.to_lowercase() == "same_bst" => Algo::SameBST,
            s if s.to_lowercase() == "selectionsort" => Algo::SelectionSort,
            s if s.to_lowercase() == "shapearea" => Algo::ShapeArea,
            s if s.to_lowercase() == "shorten_path" => Algo::ShortenPath,
            s if s.to_lowercase() == "sigma_k" => Algo::SigmaK,
            s if s.to_lowercase() == "single_cycle_check" => Algo::SingleCycleCheck,
            s if s.to_lowercase() == "single_stroke" => Algo::SingleStroke,
            s if s.to_lowercase() == "singly_linked_list_copy" => Algo::SinglyLinkedListCopy,
            s if s.to_lowercase() == "singly_linked_list_reverse" => Algo::SinglyLinkedListReverse,
            s if s.to_lowercase() == "smallest_difference" => Algo::SmallestDifference,
            s if s.to_lowercase() == "smallest_positive_product" => Algo::SmallestPositiveProduct,
            s if s.to_lowercase() == "smallest_sum" => Algo::SmallestSum,
            s if s.to_lowercase() == "sprite_index" => Algo::SpriteIndex,
            s if s.to_lowercase() == "softmax" => Algo::Softmax,
            s if s.to_lowercase() == "softsign" => Algo::Softsign,
            s if s.to_lowercase() == "sort_by_height" => Algo::SortByHeight,
            s if s.to_lowercase() == "sort_stack" => Algo::SortStack,
            s if s.to_lowercase() == "sorted_matrix_search" => Algo::SortedMatrixSearch,
            s if s.to_lowercase() == "stack_copy_only_pop" => Algo::StackCopyOnlyPop,
            s if s.to_lowercase() == "stdev" => Algo::Stdev,
            s if s.to_lowercase() == "storage_projection_for_sdhd" => Algo::StorageProjectionForSDHD,
            s if s.to_lowercase() == "strange_bank" => Algo::StrangeBank,
            s if s.to_lowercase() == "string_construction" => Algo::StringConstruction,
            s if s.to_lowercase() == "string_pattern" => Algo::StringPattern,
            s if s.to_lowercase() == "sturge_formula" => Algo::SturgeFormula,
            s if s.to_lowercase() == "subarray_sort" => Algo::SubArraySort,
            s if s.to_lowercase() == "sudoku" => Algo::Sudoku,
            s if s.to_lowercase() == "suffix_trie" => Algo::SuffixTrue,
            s if s.to_lowercase() == "sum_of_arithmatic_progression" => Algo::SumOfArithmaticProgression,
            s if s.to_lowercase() == "sum_of_consecutive" => Algo::SumOfConsective,
            s if s.to_lowercase() == "sum_of_integers" => Algo::SumOfIntegers,
            s if s.to_lowercase() == "sum_of_squared_deviations" => Algo::SumOfSquaredDeviations,
            s if s.to_lowercase() == "sunset_view" => Algo::SunsetView,
            s if s.to_lowercase() == "swap_sibling" => Algo::SwapSibling,
            s if s.to_lowercase() == "tandem_bike" => Algo::TandemBike,
            s if s.to_lowercase() == "tandemrepeat" => Algo::TandemRepeat,
            s if s.to_lowercase() == "task_assignment" => Algo::TaskAssignment,
            s if s.to_lowercase() == "three_number_sort" => Algo::ThreeNumberSort,
            s if s.to_lowercase() == "three_sum" => Algo::ThreeSum,
            s if s.to_lowercase() == "toggle_bit" => Algo::ToggleBit,
            s if s.to_lowercase() == "topological_sort" => Algo::TopologicalSort,
            s if s.to_lowercase() == "total_depth_sum_of_tree" => Algo::TotalDepthSumOfTree,
            s if s.to_lowercase() == "total_sum_with_n_nums" => Algo::TotalSumWithNNums,
            s if s.to_lowercase() == "turn_commands" => Algo::TurnCommands,
            s if s.to_lowercase() == "twosum" => Algo::TwoSum,
            s if s.to_lowercase() == "unconstructiable_number" => Algo::UnconstructiableNumber,
            s if s.to_lowercase() == "valid_parentheses" => Algo::ValidParentheses,
            s if s.to_lowercase() == "valid_starting_city" => Algo::ValidStartingCity,
            s if s.to_lowercase() == "validate_three_nodes" => Algo::ValidateThreeNodes,
            s if s.to_lowercase() == "variance" => Algo::Variance,
            s if s.to_lowercase() == "waterarea" => Algo::WaterArea,
            s if s.to_lowercase() == "xor_shift" => Algo::XOrShift,
            s if s.to_lowercase() == "yuv_to_rgb" => Algo::YuvToRgb,
            s if s.to_lowercase() == "zigzag_traversal" => Algo::zigzagTraversal,
            s if s.to_lowercase() == "zscore" => Algo::ZScore,
            _ => panic!("{} has not implemented yet", algo_str),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(version, author, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub algo: String,
}
