module Sudoku exposing (SudokuBoard, viewSudoku)

import Array exposing (Array)
import Html exposing (..)
import Html.Attributes exposing (..)


type alias SudokuBoard =
    { cells : List (List Int)
    , width : Int
    , height : Int
    }


viewSudoku : SudokuBoard -> Html msg
viewSudoku board =
    let
        viewRow : List Int -> Html msg
        viewRow =
            List.map (li [] << List.singleton << text << String.fromInt)
                >> ul []
    in
    List.map viewRow board.cells
        |> List.map (li [] << List.singleton)
        |> ul []
