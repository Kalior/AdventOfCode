app "adventofcode"
    packages { pf: "https://github.com/roc-lang/basic-cli/releases/download/0.5.0/Cufzl36_SnJ4QbOoEmiJ5dIpUxBvdB3NEySvuH82Wio.tar.br" }
    imports [
        pf.Stdout,
        pf.Stderr,
        pf.File,
        pf.Path,
        pf.Task.{ Task },
        Day2.{ solve }
    ]
    provides [main] to pf


main =
    solve
    |> Task.onErr \e -> 
        dbg e
        Stderr.line "Something went wrong!"
    