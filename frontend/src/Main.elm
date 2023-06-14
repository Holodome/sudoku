module Main exposing (main)

import Html exposing (..)
import Html.Attributes exposing (..)
import Sudoku exposing (..)

view : String -> Html msg
view data =
    div []
        [ h1 [] [ text data ]
        ]


main : Html msg
main =
    view "hello world"
