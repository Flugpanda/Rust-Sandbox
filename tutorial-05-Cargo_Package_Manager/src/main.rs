// cargo nutzt das Paket von https://crate.io
extern crate irc;

use irc::client::prelude::*;

fn main(){
	let server = IrcServer::new("config.json").unwrap();
	server.identify().unwrap();
}
