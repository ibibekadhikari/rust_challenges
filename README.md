# Rust_Challenges
Tracking everything i code in RUST inside this Repository. Initialising from the beginning so experts has no entry in here : D 

<h2>a_variables</h1>
This Directory consist of a main.rs which is created in order to Practice Variable formats. Since variables in Rust are immutable by default and it is strongly typed language. 
<em>Things to know: </em>
<ul>
 <li>let or const keywords are used inorder to declare variables.</li>
  <li>Variables are immutable by default so, <code>mut</code> keyword can be used to make it mutable.</li>
  <li>const needs to be declared in a <code>SCREAMING_SNAKE_CASE</code> while normal variables & functions can be declared with <code>snake_case</code>.  </li>
  <li>Datatype should be defined after the variable name and colon. Example <code>let intvalue : i32 </code> for integer and f64 for float.</li>
</ul>

<h2>b_functions</h2>
This directory consist of a main.rs which is created in order to Practice different sort of rules and how we define and use function. 
<em>Things to know: </em>
<ul>
  <li>We define function with keyword fn </li>
  <li>Datatype should be present inorder to use it as Argument</li>
  <li>If there is some sort of returning value inside scope then it needs to be defined ( -> DataType )</li>
</ul>

 <h2>std_input_out</h2>
This directory consist of a main.rs file which is created inorder to practice the crate (library) named of std::io which basically helps to input and output in the program.
<em>Things to know: </em>
<ul>
<li><code>use std::io;</code> is the crate which helps us to input and output the data in the porgram in RUST</li>
<li>It accepts the value in the String format therefore we need to parse to integer if we want integer as an input </li>
<li>After using <code>std::io</code>, we use io::stdin().read_line(&mut variable_name) in order to input a value and store in the variable name and var_name should be string in the first place </li>
<li>Here, <code> .expect("Message, if error occurs"); </code> can be written after <em>read_line(&mut variable_name) </em> in order to handle error for unusual input value </li>
<li>Like that, for integer we have tp trim() and then parse() and store it into i32,i62 typed variables. </li>
</ul>

<h2>custom_input</h2>
This directory consist of a main.rs file and inout_dir directory which holds mod. Mod is basically used inorder to break the large chunks of code into smaller piesces. 
<em>Things to know: </em>
<ul>
  <li>We define mod using mod keyword & to use it in different file we need to assign pub keywords which means public.</li>
  <li>We can use the mod by using mod::(directory_name) in the main file.</li>
  <li>A mod can have many functions of other mod inside it so we access function by using <code> directory_name::mod_name::function </code></li>
  <li>Mod is very useful as it helps to main piece of code, crate and mod are similar but different at the same times.</li>
  <li>This program is basically created inorder to simplfy the <code>std::io::stdin().readline()</code> to take and conver value into <code>int</code>   since it takes String only. </li>
</ul>
<--! Will begin after two weeks --!>
<!--About macros --!>
