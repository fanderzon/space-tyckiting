//
//  utils.swift
//  TyckitingClient
//
//  Created by Markus Kauppila on 04/04/15.
//  Copyright (c) 2015 Futurice. All rights reserved.
//

import Foundation

func RandomInt(range: Range<Int>) -> Int {
    let low = range.startIndex
    let high = UInt32(range.endIndex - range.startIndex)
    return low + Int(arc4random_uniform(high))
}
