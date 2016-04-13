module Basics = 
    let xValue = 2
    let yValue = xValue + 3
    let add x y = x + y
    let additionResult = add 2 3
    let addTwo arg = add arg 2
    
    addTwo 10
    
    let getTwo() = 2
    
    getTwo()

module Piping = 
    let print x = printfn "Integer %d" x
    
    print 5
    5 |> print
    
    let addTwo x = x + 2
    
    5
    |> addTwo
    |> print

module Collections = 
    let list = [ 1; 2; 3; 4 ]
    let rangeList = [ 1..4 ]
    let array = [| 1..4 |]
    
    let sequence = 
        seq { 
            for i in 1..1000000 do
                yield i
        }
    
    let yieldList = 
        [ yield 1
          yield 2
          for i in 3..10 do
              yield i
          yield 11 ]
    
    let mergedArray = list @ rangeList
    let appendedList = 6 :: list
    
    list
    |> List.map (fun x -> x + 2)
    |> List.filter (fun x -> x % 2 = 0)
    |> printfn "%A"

module Tuple = 
    let tuple = 2, 3

module Records = 
    type Person = 
        { Name : string
          Age : int }
    
    let john = 
        { Name = "John"
          Age = 27 }
    
    let olderJohn = { john with Age = 28 }

module ``Discriminated unions`` = 
    type Color = 
        | Red
        | Green
        | Blue
    
    type CreditCardId = 
        | CreditCardId of int
    
    let id = CreditCardId 23
    
    type Person = 
        { Name : string
          Age : int }
    
    type Payment = 
        | Cash
        | CreditCard of CreditCardId * Person

module Unboxing = 
    let tuple = 2, 3
    let first, second = tuple
    
    type CreditCardId = 
        | CreditCardId of int
    
    let cardId = CreditCardId 5
    let CreditCardId id = cardId
    
    type Person = 
        { Name : string
          Age : int }
    
    let john = 
        { Name = "John"
          Age = 27 }
    
    // OO
    let name = john.Name
    // Patern matching
    let { Name = _; Age = age } = john

module ``Patern matching`` = 
    let value = 4
    
    match value with
    | x when x = 4 -> printfn "Value is 4"
    | _ -> printfn "Value is not 4"
    
    type Person = 
        { Name : string
          Age : int }
    
    let john = 
        { Name = "John"
          Age = 27 }
    
    let result = 
        match john with
        | { Age = age } when age < 18 -> sprintf "Young person"
        | { Age = age } when age >= 18 -> sprintf "Adult"
        | _ -> sprintf "No Idea"
    
    type CreditCardId = 
        | CreditCardId of int
    
    type Payment = 
        | Cash
        | CreditCard of CreditCardId * Person
    
    let creditCard = CreditCard(CreditCardId 1, john)
    
    match creditCard with
    | CreditCard _ -> printfn "Paying with credit card"
    | Cash -> printfn "Paying with cash"
    
    let list = [ 5; 4; 3; 2 ]
    
    match list with
    | [] -> printfn "List is empty"
    | [ _ ] -> printfn "List has one element"
    | [ _; _ ] -> printfn "List has two elements"
    | x :: xs -> printfn "First element is %A and the rest is %A" x xs

module ``Recursive functions`` = 
    let rec printSmallerThen10 number = 
        match number with
        | n when n < 10 -> 
            printfn "%d" n
            printSmallerThen10 (n + 1)
        | _ -> ()
    
    printSmallerThen10 0
    
    let rec printList = 
        function 
        | [] -> ()
        | x :: xs -> 
            printfn "%A" x
            printList xs
    
    printList [ 1..2..30 ]

module Options = 
    let list = [ 1; 2; 3; 4 ]
    
    let option = 
        list
        |> List.tryFind ((=) 3)
        |> Option.map ((*) 3)
    
    match option with
    | Some value -> printfn "Found: %A" value
    | None -> printfn "Did not found"
