/*
 * CommandLineTests.swift
 * Copyright (c) 2014 Ben Gollmer.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import XCTest
import CommandLine

internal class CommandLineTests: XCTestCase {
  
  func testBoolOptions() {
    let cli = CommandLine(arguments: [ "CommandLineTests", "-a", "--bool", "-c", "-c", "-ddd" ])
    
    /* Short flag */
    let a = BoolOption(shortFlag: "a", longFlag: "a1", helpMessage: "")
    
    /* Long flag */
    let b = BoolOption(shortFlag: "b", longFlag: "bool", helpMessage: "")
    
    /* Multiple flags
     * Do not throw an error if a bool value is specified more than once
     */
    let c = BoolOption(shortFlag: "c", longFlag: "c1", helpMessage: "")
    
    /* Concatenated multiple flags
     * As with separate multiple flags, don't barf if this happens
     */
    let d = BoolOption(shortFlag: "d", longFlag: "d1", helpMessage: "")
    
    /* Missing flag */
    let e = BoolOption(shortFlag: "e", longFlag: "e1", helpMessage: "")

    /* Default value is set */
    let f = BoolOption(shortFlag: "f", longFlag: "f1", helpMessage: "", defaultValue: true)
    
    cli.addOptions(a, b, c, d, e, f)
    let (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse bool options")
    XCTAssertNil(error, "Non-nil parse error after successful parse")

    XCTAssertTrue(a.value, "Failed to get true value from short bool")
    XCTAssertTrue(b.value, "Failed to get true value from long bool")
    XCTAssertTrue(c.value, "Failed to get true value from multi-flagged bool")
    XCTAssertTrue(d.value, "Failed to get true value from concat multi-flagged bool")
    XCTAssertFalse(e.value, "Failed to get false value from missing bool")
    XCTAssertTrue(f.value == true, "Failed to read default value")
  }
  
  func testIntOptions() {
    let cli = CommandLine(arguments: [ "CommandLineTests", "-a", "1", "--bigs", "2", "-c", "3",
      "-c", "4", "-ddd", "-e", "bad", "-f", "-g", "-5", "h", "22" ])
    
    /* Short flag */
    let a = IntOption(shortFlag: "a", longFlag: "a1", required: false, helpMessage: "")
    
    /* Long flag */
    let b = IntOption(shortFlag: "b", longFlag: "bigs", required: false, helpMessage: "")
    
    /* Multiple short flags
     * If an int is specified multiple times, return the last (rightmost) value
     */
    let c = IntOption(shortFlag: "c", longFlag: "c1", required: false, helpMessage: "")
    /* Default value */
    let h = IntOption(shortFlag: "h", longFlag: "h1", required: false, helpMessage: "", defaultValue: 5)
    
    cli.addOptions(a, b, c, h)
    var (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse int options")
    XCTAssertNil(error, "Non-nil parse error after successful parse")
    XCTAssertEqual(a.value, 1, "Failed to get correct value from short int")
    XCTAssertEqual(b.value, 2, "Failed to get correct value from long int")
    XCTAssertEqual(c.value, 4, "Failed to get correct value from multi-flagged int")
    XCTAssertEqual(h.value, 5, "Failed to match correct default value")
    
    /* Concatenated multiple flags
     * Concat flags can't have values
     */
    let d = IntOption(shortFlag: "d", longFlag: "d1", required: false, helpMessage: "")
    cli.setOptions(d)
    (success, error) = cli.parse()
    XCTAssertFalse(success, "Parsed invalid concat int option")
    XCTAssertNotNil(error, "No parse error after parsing failed")
    //XCTAssertNil(d.value, "Got non-nil value from concat multi-flagged int")
    
    /* Non-int value */
    let e = IntOption(shortFlag: "e", longFlag: "e1", required: false, helpMessage: "")
    cli.setOptions(e)
    (success, error) = cli.parse()
    XCTAssertFalse(success, "Parsed invalid int option")
    XCTAssertNotNil(error, "No parse error after parsing failed")
    //XCTAssertNil(e.value, "Got non-nil value from invalid int")
    
    /* No value */
    let f = IntOption(shortFlag: "f", longFlag: "f1", required: false, helpMessage: "")
    cli.setOptions(f)
    (success, error) = cli.parse()
    XCTAssertFalse(success, "Parsed int option with no value")
    XCTAssertNotNil(error, "No parse error after parsing failed")
    //XCTAssertNil(f.value, "Got non-nil value from no value int")
    
    /* Negative int */
    let g = IntOption(shortFlag: "g", longFlag: "g1", required: false, helpMessage: "")
    cli.setOptions(g)
    cli.parse()
    (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse int option with negative value")
    XCTAssertNil(error, "Non-nil parse error after successful parse")
    XCTAssertEqual(g.value, -5, "Failed to get correct value from int option with negative value")
  }
  
  func testCounterOptions() {
    let cli = CommandLine(arguments: [ "CommandLineTests", "-a", "--bach", "-c", "-c",
      "--doggerel", "-doggerel", "--doggerel", "-eeee"])
    
    /* Short flag */
    let a = CounterOption(shortFlag: "a", longFlag: "a1", helpMessage: "")
    
    /* Long flag */
    let b = CounterOption(shortFlag: "b", longFlag: "bach", helpMessage: "")
    
    /* Multiple short flags
     * If a double is specified multiple times, return the last (rightmost) value
     */
    let c = CounterOption(shortFlag: "c", longFlag: "c1", helpMessage: "")
    
    /* Multiple long flags */
    let d = CounterOption(shortFlag: "d", longFlag: "doggerel", helpMessage: "")
    
    /* Concatenated multiple flags */
    let e = CounterOption(shortFlag: "e", longFlag: "e1", helpMessage: "")
    
    /* Unspecified option should return 0, not nil */
    let f = CounterOption(shortFlag: "f", longFlag: "f1", helpMessage: "")
    
    cli.addOptions(a, b, c, d, e, f)
    let (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse counter options")
    XCTAssertNil(error, "Non-nil parse error after successful parse")
    XCTAssertEqual(a.value, 1, "Failed to get correct value from short counter")
    XCTAssertEqual(b.value, 1, "Failed to get correct value from long counter")
    XCTAssertEqual(c.value, 2, "Failed to get correct value from multi-flagged short counter")
    XCTAssertEqual(d.value, 3, "Failed to get correct value from multi-flagged long counter")
    XCTAssertEqual(e.value, 4, "Failed to get correct value from concat multi-flagged counter")
    XCTAssertEqual(f.value, 0, "Failed to get correct value from unspecified counter")
  }
  
  func testDoubleOptions() {
    let cli = CommandLine(arguments: [ "CommandLineTests", "-a", "1.4", "--baritone", "2.5",
      "-c", "5.0", "-c", "5.2", "--dingus", "8.5", "--dingus", "8.8", "-e", "95",
      "-f", "bad", "-g", "-h", "-3.14159" ])
    
    /* Short flag */
    let a = DoubleOption(shortFlag: "a", longFlag: "a1", required: true, helpMessage: "")
    
    /* Long flag */
    let b = DoubleOption(shortFlag: "b", longFlag: "baritone", required: true, helpMessage: "")
    
    /* Multiple short flags */
    let c = DoubleOption(shortFlag: "c", longFlag: "c1", required: true, helpMessage: "")
    
    /* Multiple long flags */
    let d = DoubleOption(shortFlag: "d", longFlag: "dingus", required: true, helpMessage: "")
    
    /* Integer value */
    let e = DoubleOption(shortFlag: "e", longFlag: "e1", required: true, helpMessage: "")

    /* Default value value */
    let i = DoubleOption(shortFlag: "e", longFlag: "e1", required: false, helpMessage: "", defaultValue: 5.0)


    cli.addOptions(a, b, c, d, e, i)
    var (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse double options")
    XCTAssertNil(error, "Non-nil parse error after successful parse")
    XCTAssertEqual(a.value, 1.4, "Failed to get correct value from short double")
    XCTAssertEqual(b.value, 2.5, "Failed to get correct value from long double")
    XCTAssertEqual(c.value, 5.2, "Failed to get correct value from multi-flagged short double")
    XCTAssertEqual(d.value, 8.8, "Failed to get correct value from multi-flagged long double")
    XCTAssertEqual(e.value, 95.0, "Failed to get correct double value from integer argument")
    XCTAssertEqual(i.value, 5.0, "Failed to match correct default value")


    /* Non-double value */
    let f = DoubleOption(shortFlag: "f", longFlag: "f1", required: true, helpMessage: "")
    cli.setOptions(f)
    (success, error) = cli.parse()
    XCTAssertFalse(success, "Parsed invalid double option")
    XCTAssertNotNil(error, "No parse error after parsing failed")
//    XCTAssertNil(f.value, "Got non-nil value from invalid double")
    
    /* No value */
    let g = DoubleOption(shortFlag: "g", longFlag: "g1", required: true, helpMessage: "")
    cli.setOptions(g)
    (success, error) = cli.parse()
    XCTAssertFalse(success, "Parsed double option with no value")
    XCTAssertNotNil(error, "No parse error after parsing failed")
  //  XCTAssertNil(g.value, "Got non-nil value from no value double")
    
    /* Negative double */
    let h = DoubleOption(shortFlag: "h", longFlag: "h1", required: true, helpMessage: "")
    cli.setOptions(h)
    (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse double option with negative value")
    XCTAssertNil(error, "Non-nil parse error after successful parse")
    XCTAssertEqual(h.value, -3.14159, "Failed to get correct value from double with negative value")
  }
  
  func testDoubleOptionsInAlternateLocale() {
    let cli = CommandLine(arguments: ["CommandLineTests", "-a", "3,14159"])
    let a = DoubleOption(shortFlag: "a", longFlag: "a1", required: true, helpMessage: "")
    
    cli.addOptions(a)
    
    setlocale(LC_ALL, "sv_SE.UTF-8")
    let (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse double options in alternate locale")
    XCTAssertNil(error, "Non-nil parse error after successful parse")
    XCTAssertEqual(a.value, 3.14159, "Failed to get correct value from double in alternate locale")
  }
  
  func testStringOptions() {
    let cli = CommandLine(arguments: [ "CommandLineTests", "-a", "one", "--b1", "two", "-c", "x", "-c", "xx",
      "--d1", "y", "--d1", "yy", "-e" ])
    
    /* Short flag */
    let a = StringOption(shortFlag: "a", longFlag: "a1", required: true, helpMessage: "")
    
    /* Long flag */
    let b = StringOption(shortFlag: "b", longFlag: "b1", required: true, helpMessage: "")
    
    /* Multiple short flags */
    let c = StringOption(shortFlag: "c", longFlag: "c1", required: true, helpMessage: "")
    
    /* Multiple long flags */
    let d = StringOption(shortFlag: "d", longFlag: "d1", required: true, helpMessage: "")

    /* Default value */
    let f = StringOption(shortFlag: "f", longFlag: "f1", required: false, helpMessage: "", defaultValue: "default")
    
    cli.addOptions(a, b, c, d, f)
    var (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse string options")
    XCTAssertNil(error, "Non-nil parse error after successful parse")
    XCTAssertEqual(a.value, "one", "Failed to get correct value from short string")
    XCTAssertEqual(b.value, "two", "Failed to get correct value from long string")
    XCTAssertEqual(c.value, "xx", "Failed to get correct value from multi-flagged short string")
    XCTAssertEqual(d.value, "yy", "Failed to get correct value from multi-flagged long string")
    XCTAssertEqual(f.value, "default", "Failed to get correct default value")

    /* No value */
    let e = StringOption(shortFlag: "e", longFlag: "e1", required: false, helpMessage: "")
    cli.setOptions(e)
    (success, error) = cli.parse()
    XCTAssertFalse(success, "Parsed string option with no value")
    XCTAssertNotNil(error, "No parse error after parsing failed")
//    XCTAssertNil(e.value, "Got non-nil value from no value string")
  }
  
  func testMultiStringOptions() {
    let cli = CommandLine(arguments: [ "CommandLineTests", "-a", "one", "-b", "two", "2wo",
      "--c1", "three", "--d1", "four", "4our", "-e" ])
    
    /* Short flags */
    let a = MultiStringOption(shortFlag: "a", longFlag: "a1", required: true, helpMessage: "")
    let b = MultiStringOption(shortFlag: "b", longFlag: "b1", required: true, helpMessage: "")
    
    /* Long flags */
    let c = MultiStringOption(shortFlag: "c", longFlag: "c1", required: true, helpMessage: "")
    let d = MultiStringOption(shortFlag: "d", longFlag: "d1", required: true, helpMessage: "")

    let f = MultiStringOption(shortFlag: "f", longFlag: "f1", required: false, helpMessage: "", defaultValue: ["default1", "default2"])
    
    cli.addOptions(a, b, c, d, f)
    var (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse multi string options")
    XCTAssertNil(error, "Non-nil parse error after successful parse")

    XCTAssertEqual(a.value.count, 1, "Failed to get correct number of values from single short multistring")
    XCTAssertEqual(a.value[0], "one", "Filed to get correct value from single short multistring")

    XCTAssertEqual(b.value.count, 2, "Failed to get correct number of values from multi short multistring")
    XCTAssertEqual(b.value[0], "two", "Failed to get correct first value from multi short multistring")
    XCTAssertEqual(b.value[1], "2wo", "Failed to get correct second value from multi short multistring")

    XCTAssertEqual(c.value.count, 1, "Failed to get correct number of values from single long multistring")
    XCTAssertEqual(c.value[0], "three", "Filed to get correct value from single long multistring")

    XCTAssertEqual(d.value.count, 2, "Failed to get correct number of values from multi long multistring")
    XCTAssertEqual(d.value[0], "four", "Failed to get correct first value from multi long multistring")
    XCTAssertEqual(d.value[1], "4our", "Failed to get correct second value from multi long multistring")

    XCTAssertEqual(f.value.count, 2, "Failed to get correct number of values from multi long multistring")
    XCTAssertEqual(f.value[0], "default1", "Failed to get correct first value from multi long multistring")
    XCTAssertEqual(f.value[1], "default2", "Failed to get correct second value from multi long multistring")

    /* No value */
    let e = MultiStringOption(shortFlag: "e", longFlag: "e1", required: false, helpMessage: "")
    cli.setOptions(e)
    (success, error) = cli.parse()
    XCTAssertFalse(success, "Parsed multi string option with no value")
    XCTAssertNotNil(error, "No parse error after parsing failed")
//    XCTAssertNil(e.value, "Got non-nil value from no value multistring")
  }

  func testConcatOptionWithValue() {
    let cli = CommandLine(arguments: [ "CommandLineTests", "-xvf", "file1", "file2" ])
    
    let x = BoolOption(shortFlag: "x", longFlag: "x1", helpMessage: "")
    let v = CounterOption(shortFlag: "v", longFlag: "v1", helpMessage: "")
    let f = MultiStringOption(shortFlag: "f", longFlag: "file", required: true, helpMessage: "")
    
    cli.addOptions(x, v, f)
    let (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse concat flags with value")
    XCTAssertNil(error, "Non-nil parse error after successful parse")
    XCTAssertTrue(x.value as Bool, "Failed to get true value from concat flags with value")
    XCTAssertEqual(v.value, 1, "Failed to get correct value from concat flags with value")
    XCTAssertEqual(f.value.count, 2, "Failed to get values from concat flags with value")
    XCTAssertEqual(f.value[0], "file1", "Failed to get first value from concat flags with value")
    XCTAssertEqual(f.value[1], "file2", "Failed to get second value from concat flags with value")
  }
  
  func testMissingRequiredOption() {
    let cli = CommandLine(arguments: [ "CommandLineTests", "-a", "-b", "foo", "-q", "quux" ])
    let c = StringOption(shortFlag: "c", longFlag: "c1", required: true, helpMessage: "")

    cli.addOption(c)
    let (success, error) = cli.parse()
    XCTAssertFalse(success, "Parsed missing required option")
    XCTAssertNotNil(error, "No parse error after parsing failed")
    //XCTAssertNil(c.value, "Got non-nil value from missing option")
  }
  
  func testAttachedArgumentValues() {
    let cli = CommandLine(arguments: [ "CommandLineTests", "-a=5", "--bb=klaxon" ])
    
    let a = IntOption(shortFlag: "a", longFlag: "a1", required: true, helpMessage: "")
    let b = StringOption(shortFlag: "b", longFlag: "bb", required: true, helpMessage: "")
    
    cli.addOptions(a, b)
    let (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse attached argument values")
    XCTAssertNil(error, "Non-nil parse error after successful parse")
    XCTAssertEqual(a.value, 5, "Failed to get correct int attached value")
    XCTAssertEqual(b.value, "klaxon", "Failed to get correct string attached value")
  }
  
  func testEmojiOptions() {
    let cli = CommandLine(arguments: [ "CommandLineTests", "-👻", "3", "--👍", "☀️" ])
    
    let a = IntOption(shortFlag: "👻", longFlag: "👻", required: true, helpMessage: "")
    let b = StringOption(shortFlag: "👍", longFlag: "👍", required: true, helpMessage: "")
    
    
    cli.addOptions(a, b)
    let (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse emoji options")
    XCTAssertNil(error, "Non-nil parse error after successful parse")
    XCTAssertEqual(a.value, 3)
    XCTAssertEqual(b.value, "☀️")
  }
  
  func testEnumOption() {
    enum Operation: String {
      case Create = "c"
      case Extract = "x"
      case List = "l"
      case Verify = "v"
    }
    
    let cli = CommandLine(arguments: [ "CommandLineTests", "--operation", "x" ])
    let op = EnumOption<Operation>(shortFlag: "o", longFlag: "operation", required: true, helpMessage: "")
    
    cli.setOptions(op)
    let (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse enum options")
    XCTAssertNil(error, "Non-nil parse error after successful parse")
    XCTAssertEqual(op.value!, Operation.Extract, "Failed to get correct value from enum option")
  }
  
  func testArgumentStopper() {
    let cli = CommandLine(arguments: [ "CommandLineTests", "-a", "--", "-value", "--", "-55" ])
    let op = MultiStringOption(shortFlag: "a", longFlag: "a1", required: true, helpMessage: "")
    
    cli.setOptions(op)
    let (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse options with an argument stopper")
    XCTAssertNil(error, "Non-nil parse error after successful parse")
    XCTAssertEqual(op.value.count, 3, "Failed to get correct number of options with stopper")
    XCTAssertEqual(op.value[0], "-value", "Failed to get correct value from options with stopper")
    XCTAssertEqual(op.value[1], "--", "Failed to get correct value from options with stopper")
    XCTAssertEqual(op.value[2], "-55", "Failed to get correct value from options with stopper")
  }
  
  func testFlagStyles() {
    let argLines = [
      [ "CommandLineTests", "-xvf", "/path/to/file" ],
      [ "CommandLineTests", "-x", "-v", "-f", "/path/to/file" ],
      [ "CommandLineTests", "-x", "--verbose", "--file", "/path/to/file" ],
      [ "CommandLineTests", "-xv", "--file", "/path/to/file" ],
      [ "CommandLineTests", "--extract", "-v", "--file=/path/to/file" ]
    ]
    
    for args in argLines {
      let cli = CommandLine(arguments: args)
      let extract = BoolOption(shortFlag: "x", longFlag: "extract", helpMessage: "")
      let verbosity = CounterOption(shortFlag: "v", longFlag: "verbose", helpMessage: "")
      let filePath = StringOption(shortFlag: "f", longFlag: "file", required: true, helpMessage: "")
      
      cli.setOptions(extract, verbosity, filePath)
      let (success, error) = cli.parse()
      XCTAssertTrue(success, "Failed to parse arg line \(args)")
      XCTAssertNil(error, "Non-nil parse error after successful parse")
      XCTAssertEqual(extract.value, true, "Failed to parse extract value from arg line \(args)")
      XCTAssertEqual(verbosity.value, 1, "Failed to parse verbosity value from arg line \(args)")
      XCTAssertEqual(filePath.value, "/path/to/file", "Failed to parse file path value from arg line \(args)")
    }
  }
  
  func testMixedExample() {
    let cli = CommandLine(arguments: [ "CommandLineTests", "-dvvv", "--name", "John Q. Public",
      "-f", "45", "-p", "0.05", "-x", "extra1", "extra2", "extra3" ])
    
    let boolOpt = BoolOption(shortFlag: "d", longFlag: "debug", helpMessage: "Enables debug mode.")
    
    let counterOpt = CounterOption(shortFlag: "v", longFlag: "verbose",
      helpMessage: "Enables verbose output. Specify multiple times for extra verbosity.")
    let stringOpt = StringOption(shortFlag: "n", longFlag: "name", required: true,
      helpMessage: "Name a Cy Young winner.")
    let intOpt = IntOption(shortFlag: "f", longFlag: "favorite", required: true,
      helpMessage: "Your favorite number.")
    let doubleOpt = DoubleOption(shortFlag: "p", longFlag: "p-value", required: true,
      helpMessage: "P-value for test.")
    let extraOpt = MultiStringOption(shortFlag: "x", longFlag: "Extra", required: true,
      helpMessage: "X is for Extra.")
    
    cli.addOptions(boolOpt, counterOpt, stringOpt, intOpt, doubleOpt, extraOpt)
    let (success, error) = cli.parse()
    XCTAssertTrue(success, "Failed to parse mixed command line")
    XCTAssertNil(error, "Non-nil parse error after successful parse")
    XCTAssertTrue(boolOpt.value, "Failed to get correct bool value from mixed command line")
    XCTAssertEqual(counterOpt.value, 3, "Failed to get correct counter value from mixed command line")
    XCTAssertEqual(stringOpt.value, "John Q. Public", "Failed to get correct string value from mixed command line")
    XCTAssertEqual(intOpt.value, 45, "Failed to get correct int value from mixed command line")
    XCTAssertEqual(doubleOpt.value, 0.05, "Failed to get correct double value from mixed command line")
    XCTAssertEqual(extraOpt.value.count  , 3, "Failed to get correct number of multistring options from mixed command line")
  }
}
