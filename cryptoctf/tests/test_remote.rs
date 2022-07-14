extern crate cryptoctf;

#[cfg(test)]
mod test {

    use cryptoctf::remote::Remote;

    #[test]
    fn test_remote() {
        let mut r = Remote::new("www.antoxyde.fr:80");
        r.sendline("GET / HTTP/1.0\r");
        let s = r.read_to_string();
        println!("Response : {:?}", s);
    }
}
