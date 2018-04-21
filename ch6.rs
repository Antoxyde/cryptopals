/*
Cryptoptals stage 1 challenge 6 solution
https://www.cryptopals.com/sets/1/challenges/6

Output :
[antoxyde@anarchy-fixe:Projets/Cryptopals][127]$ rustc ch6.rs
warning: unused `std::result::Result` which must be used
  --> ch6.rs:24:5
   |
24 |     file.unwrap().read_to_string(&mut encoded_content);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default

[antoxyde@anarchy-fixe:Projets/Cryptopals]$ ./ch6
The key is : "Terminator X: Bring the noise" and the plaintext is  :
I'm back and I'm ringin' the bell
A rockin' on the mike while the fly girls yell
In ecstasy in the back of me
Well that's my DJ Deshay cuttin' all them Z's
Hittin' hard and the girlies goin' crazy
Vanilla's on the mike, man I'm not lazy.

I'm lettin' my drug kick in
It controls my mouth and I begin
To just let it flow, let my concepts go
My posse's to the side yellin', Go Vanilla Go!

Smooth 'cause that's the way I will be
And if you don't give a damn, then
Why you starin' at me
So get off 'cause I control the stage
There's no dissin' allowed
I'm in my own phase
The girlies sa y they love me and that is ok
And I can dance better than any kid n' play

Stage 2 -- Yea the one ya' wanna listen to
It's off my head so let the beat play through
So I can funk it up and make it sound good
1-2-3 Yo -- Knock on some wood
For good luck, I like my rhymes atrocious
Supercalafragilisticexpialidocious
I'm an effect and that you can bet
I can take a fly girl and make her wet.

I'm like Samson -- Samson to Delilah
There's no denyin', You can try to hang
But you'll keep tryin' to get my style
Over and over, practice makes perfect
But not if you're a loafer.

You'll get nowhere, no place, no time, no girls
Soon -- Oh my God, homebody, you probably eat
Spaghetti with a spoon! Come on and say it!

VIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino
Intoxicating so you stagger like a wino
So punks stop trying and girl stop cryin'
Vanilla Ice is sellin' and you people are buyin'
'Cause why the freaks are jockin' like Crazy Glue
Movin' and groovin' trying to sing along
All through the ghetto groovin' this here song
Now you're amazed by the VIP posse.

Steppin' so hard like a German Nazi
Startled by the bases hittin' ground
There's no trippin' on mine, I'm just gettin' down
Sparkamatic, I'm hangin' tight like a fanatic
You trapped me once and I thought that
You might have it
So step down and lend me your ear
'89 in my time! You, '90 is my year.

You're weakenin' fast, YO! and I can tell it
Your body's gettin' hot, so, so I can smell it
So don't be mad and don't be sad
'Cause the lyrics belong to ICE, You can call me Dad
You're pitchin' a fit, so step back and endure
Let the witch doctor, Ice, do the dance to cure
So come up close and don't be square
You wanna battle me -- Anytime, anywhere

You thought that I was weak, Boy, you're dead wrong
So come on, everybody and sing this song

Say -- Play that funky music Say, go white boy, go white boy go
play that funky music Go white boy, go white boy, go
Lay down and boogie and play that funky music till you die.

Play that funky music Come on, Come on, let me hear
Play that funky music white boy you say it, say it
Play that funky music A little louder now
Play that funky music, white boy Come on, Come on, Come on
Play that funky music
n.

*/
pub mod set01;

use std::fs::File;
use std::io::prelude::*;

use set01::base64::base64_decode;
use set01::others::{hamming_distance, m_split};
use set01::xor::{crack_xor_key, key_cycling_xor};

fn main() {
    assert_eq!(
        hamming_distance("this is a test".to_string(), "wokka wokka!!!".to_string()),
        37
    );

    let file = File::open("6.txt");
    let mut encoded_content = String::new();
    file.unwrap().read_to_string(&mut encoded_content);

    let content: String =
        String::from_utf8(base64_decode(encoded_content.replace("\n", ""))).unwrap();

    let mut keysize_guess: Vec<(i32, i32)> = Vec::new();

    for guess in 2..40 as usize {
        //Could be optimized ? it takes several seconds ..
        let splitted = m_split(&content, guess); //make blocks of guess size;
        let mut counter = 0;
        let mut hd = 0;
        for x in &splitted {
            // loop over thoses blocks and compute their hamming distance
            for y in &splitted {
                if x != y {
                    //compute the hamming distance of the same blocks would be idiot ?
                    hd += hamming_distance(x.to_string(), y.to_string());
                    counter += 1;
                }
            }
        }

        keysize_guess.push((guess as i32, (hd / counter) / guess as i32));
    }

    keysize_guess.sort_by(|a, b| a.1.cmp(&b.1)); //Sort by lower hamming distance
    let keysize: i32 = keysize_guess[0].0; //Take the lowest one

    let mut zipped: Vec<String> = Vec::new();

    for _ in 0..keysize {
        //init a vector of string
        zipped.push(String::new());
    }

    for (c, item) in content.chars().enumerate() {
        //zip our strings between themselves
        zipped[c % keysize as usize].push(item);
    }

    let mut key: Vec<u8> = Vec::new();

    for z in zipped {
        //crack each zip one by one (which correspond to each key char)
        key.push(crack_xor_key(z.into_bytes()).unwrap())
    }

    let u_content = content.clone().into_bytes();

    let plaintext = String::from_utf8(key_cycling_xor(&u_content, &key)).unwrap(); //Decrypt the content with the key we jsut found

    println!(
        "The key is : \"{}\" and the plaintext is  : \n{}.",
        String::from_utf8(key).unwrap(),
        plaintext
    );
}
