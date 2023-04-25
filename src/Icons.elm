module Icons exposing (checkCircle, chevronDown, chevronLeft, chevronRight, chevronUp, circle, github, info, mail, menu, telegram, x, devLogo, blogLogo)

import Html exposing (Html)
import Color exposing (rgb255)
import TypedSvg exposing (line, path, polyline, svg, g)
import TypedSvg.Attributes exposing (class, cx, cy, d, points, r, viewBox, x1, x2, y1, y2, width, height, viewBox, transform, fill)
import TypedSvg.Core exposing (Svg)
import TypedSvg.Types exposing (ClipPath(..), Fill(..), Transform(..), Scale(..), Fill(..), px, pt)
import TypedSvg exposing (style)


svgIcon : String -> List (Svg msg) -> Html msg
svgIcon className =
    svg
        [ class [ "icon ", className ]
        , viewBox 0 0 24 24
        ]


checkCircle : Html msg
checkCircle =
    svgIcon "check-circle"
        [ path [ d "M22 11.08V12a10 10 0 1 1-5.93-9.14" ] []
        , polyline [ points [ ( 22, 4 ), ( 12, 14.01 ), ( 9, 11.01 ) ] ] []
        ]


chevronDown : Html msg
chevronDown =
    svgIcon "chevron-down"
        [ polyline [ points [ ( 6, 9 ), ( 12, 15 ), ( 18, 9 ) ] ] []
        ]


chevronUp : Html msg
chevronUp =
    svgIcon "chevron-up"
        [ polyline [ points [ ( 18, 15 ), ( 12, 9 ), ( 6, 15 ) ] ] []
        ]


chevronLeft : Html msg
chevronLeft =
    svgIcon "chevron-left"
        [ polyline [ points [ ( 15, 18 ), ( 9, 12 ), ( 15, 6 ) ] ] []
        ]


chevronRight : Html msg
chevronRight =
    svgIcon "chevron-right"
        [ polyline [ points [ ( 9, 18 ), ( 15, 12 ), ( 9, 6 ) ] ] []
        ]


circle : Html msg
circle =
    svgIcon "circle"
        [ TypedSvg.circle [ cx (px 12), cy (px 12), r (px 10) ] []
        ]


info : Html msg
info =
    svgIcon "info"
        [ TypedSvg.circle [ cx (px 12), cy (px 12), r (px 10) ] []
        , line [ x1 (px 12), y1 (px 16), x2 (px 12), y2 (px 12) ] []
        , line [ x1 (px 12), y1 (px 8), x2 (px 12), y2 (px 8) ] []
        ]


github : Html msg
github =
    svgIcon "github"
        [ path [ d "M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22" ] []
        ]


mail : Html msg
mail =
    svgIcon "mail"
        [ path [ d "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" ] []
        , polyline [ points [ ( 22, 6 ), ( 12, 13 ), ( 2, 6 ) ] ] []
        ]


menu : Html msg
menu =
    svgIcon "menu"
        [ line [ x1 (px 3), y1 (px 12), x2 (px 21), y2 (px 12) ] []
        , line [ x1 (px 3), y1 (px 6), x2 (px 21), y2 (px 6) ] []
        , line [ x1 (px 3), y1 (px 18), x2 (px 21), y2 (px 18) ] []
        ]


telegram : Html msg
telegram =
    svgIcon "telegram"
        [ path [ d "M23.932 3.769l-3.622 17.08c-.273 1.205-.986 1.505-1.999.937l-5.518-4.066-2.663 2.561c-.294.294-.541.541-1.109.541l.397-5.62L19.646 5.96c.444-.397-.097-.616-.692-.22L6.31 13.702.867 11.998c-1.184-.37-1.205-1.184.247-1.752l21.291-8.203c.985-.369 1.848.22 1.527 1.726z" ] []
        ]


x : Html msg
x =
    svgIcon "x"
        [ line [ x1 (px 18), y1 (px 6), x2 (px 6), y2 (px 18) ] []
        , line [ x1 (px 6), y1 (px 6), x2 (px 18), y2 (px 18) ] []
        ]

-- blog logo
devLogo : Html msg
devLogo =
    svg
        [ class [ "icon" ]
        , viewBox 0 0 310 310
        , width (pt 310)
        , height (pt 310)
        ]
        [ 
            g [ transform [ Translate 0 310 
                          , Scale 0.1 -0.1
                          ] 
              , fill <| Fill <| rgb255 179 193 217
              ]
              [
                path [ d "M481 3833 c0 -5 168 -384 373 -843 l372 -835 512 -3 c282 -1 512 0 512 3 0 6 -699 1570 -736 1649 l-18 36 -508 0 c-279 0 -508 -3 -507 -7z" ] []
              , path [ d "M1064 1998 c-9 -18 -250 -471 -536 -1006 -285 -536 -518 -975 -518 -978 0 -2 450 -4 1000 -4 550 0 1000 1 1000 3 0 1 -201 454 -448 1007 -311 699 -452 1006 -464 1008 -11 2 -23 -9 -34 -30z"] []
              , path [ d"M1290 2016 c0 -2 200 -453 445 -1003 l444 -998 516 -3 c331 -1 516 1 513 8 -1 5 -203 457 -448 1004 l-445 996 -512 0 c-282 0 -513 -2 -513 -4z" ] []
              ]
        ]

blogLogo : Html msg
blogLogo =
    svg
        [ class [ "icon" ]
        , viewBox 0 0 512 512
        , width (pt 512)
        , height (pt 512)
        ]
        [
            path [ fill <| Fill <| rgb255 179 193 217,  d "M234 0.424377C187.971 6.47089 146.604 19.3708 108 46.0255C81.9464 64.0145 59.5176 87.7135 42 114C25.082 139.387 12.4017 168.111 6 198C-2.71195 238.675 -2.71447 281.64 7.88426 322C41.0576 448.325 167.099 527.849 295 509.715C330.664 504.658 366.003 491.232 396 471.333C427.65 450.336 454.472 423.291 474.575 391C543.658 280.033 514.359 130.13 411 51.2346C385.657 31.8902 356.616 17.8084 326 9.14044C297.196 0.985626 263.893 -3.50244 234 0.424377z"]  []
            -- inner text color matchs in background 
            , path [ fill <| Fill <| rgb255 142 168 191, d "M218 169C207.367 171.05 196.987 179.723 187 184.138C170.477 191.443 153.823 195.154 136 197.285C124.749 198.631 109.253 199.747 100.019 191.775C79.6214 174.168 116.33 148.489 130 140.862C171.678 117.608 226.226 114.723 272 126C254.629 147.904 241.932 173.517 228.691 198C206.625 238.8 186.423 281.257 173.71 326C167.944 346.293 155.46 393.98 185 401C176.399 383.889 185.524 358.383 190.116 341C204.334 287.171 232.278 236.041 261.424 189C270.007 175.148 279.041 161.749 289.199 149C292.323 145.08 296.294 137.159 301.171 135.353C308.328 132.703 326.692 142.704 334 144.975C354.371 151.305 374.849 155.156 396 157.17C406.317 158.152 420.752 158.618 430 152.995C438.761 147.667 447.139 134.076 437.786 125.39C432.319 120.313 422.199 118.588 415 118L415 119C424.66 121.843 447.512 143.29 426 144.91C423.692 145.083 421.308 145.068 419 144.91C382.337 142.402 348.076 127.858 312 123C314.832 100.069 290.874 114.874 280 116.711C271.996 118.062 260.103 115.64 252 114.576C238.245 112.77 223.759 113.968 210 114.91C166.818 117.868 103.614 123.755 77.5478 164C60.8698 189.75 77.7536 209.998 106 209.998C134.741 209.998 163.498 201.029 189 188.247C198.703 183.384 211.271 177.719 218 169z" ] []
        ]
