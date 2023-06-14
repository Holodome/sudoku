module Sudoku exposing (SudokuBoard)

import Array exposing (Array)
import Html exposing (..)
import Html.Attributes exposing (..)


type alias SudokuCells =
    Array Int


type alias Location =
    ( Int, Int )


type alias SudokuBoard =
    { cells : SudokuCells
    , width : Int
    , height : Int
    }


locationToIndex : SudokuBoard -> Location -> Int
locationToIndex board ( x, y ) =
    y * board.width + x


sudokuAt : SudokuBoard -> Location -> Maybe Int
sudokuAt board loc =
    Array.get (locationToIndex board loc) board.cells


sudokuSet : SudokuBoard -> Location -> Int -> SudokuBoard
sudokuSet board loc value =
    { board | cells = Array.set (locationToIndex board loc) value board.cells }


viewSudokuCells : Int -> SudokuCells -> Int -> List (Html msg)
viewSudokuCells width cells base =
    let
        nextBase =
            base + width

        row =
            Array.slice base nextBase cells
    in
    if Array.isEmpty row then
        []

    else
        (row
            |> Array.toList
            |> List.map (li [] << List.singleton << text << String.fromInt)
            |> ul []
        )
            :: viewSudokuCells width cells nextBase


viewSudoku : SudokuBoard -> Html msg
viewSudoku board =
    let
        rows =
            viewSudokuCells board.width board.cells 0
    in
    rows
        |> List.map (li [] << List.singleton)
        |> ul []
