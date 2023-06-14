module Main exposing (main)

import Html exposing (..)
import Html.Attributes exposing (..)


view model =
    div []
        [ h1 [] [ text "hello world" ]
        ]


main =
    view "hello world"
