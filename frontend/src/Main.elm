module Main exposing (main)

import Array
import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import List exposing (head)
import Sudoku exposing (..)



-- MAIN


main =
    Browser.sandbox { init = init, update = update, view = view }



-- MODEL


type alias Model =
    SudokuBoard


init : Model
init =
    let
        width: Int 
        width =
            9

        height : Int
        height =
            9
    in
    { cells = List.repeat height (List.repeat width 0)
    , width = width
    , height = height
    }



-- UPDATE


update : msg -> Model -> Model
update _ model =
    model



-- VIEW


view : Model -> Html msg
view =
    viewSudoku
