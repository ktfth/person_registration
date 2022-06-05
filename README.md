# Person Registration

## Description

On Brazil we have two main documents to describe a company or a physical person
registration, used widely for common ways like buy, register on another services and
so on. This tool help you to generate this documents to test your applications.

## Installation

```
[sudo] cargo install person_registration
```

or add to Cargo.toml

```
[dependencies]
person_registration = "*"
```

## Usage

To generate documents on cli, you can use the commands below:

### CLI

```
person_registration --physical 5
```

```
person_registration --juridic 5
```

```
person_registration --misc 5
```

### API

Or just calling on your code:

```
use person_registration::{Gen, Person, Juridic};

println!("{}", Person::generate());
println!("{}", Juridic::generate());
```

That's it, enjoy and contributions are welcome.