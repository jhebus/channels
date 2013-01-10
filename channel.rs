/*
 * This is a first attempt at dynamic Rust channels
 * 
 * Paul Harvey  
 * 8 jan 2013
 */

/*Macros?
rules_macro! chan_eq(
	
)
*/

//This is what will go accross channels
struct Message<T> {
	src		:Channel,
	dest	:Channel,
	payload	:T
}

enum direction{
	in, 
	out,
}

/*
 * Structure of a name
 *
 */
struct Name{
	mut location:~str,
	mut actor:~str,
	mut chan_name:~str,
}

/*
 * Structure of a channel 
 * 
 * NAME => location:actor:channel_name
 * TYPE => [short, int, long, unsigned ....] || custom structure
 * CONNECTIONS => should this be a table in the system, rather than per actor.
 *      This would make it smaller, but makes migration a little more difficult.
 */
struct Channel<T>{
    mut name		:Name,
	mut msg_type	:T,
    mut connections	:~[@Channel<T>],
	mut buffer		:~[Message],
	mut buff_limit	:uint,
		dir			:direction,	
}
//-----------------------------------------------------------------------------
fn same_direction(a: direction, b: direction) -> bool{
	match a{
		in => {
			match b {
				in  => {return true;}
				out => {return false;}
			}
		}
		out => {
			match b {
				in  => {return false;}
				out => {return true;}
			}

		} 
	}
}
//****************************************************************************
//DEBUG
//****************************************************************************
fn get_type(t:Msg_Type) -> ~str {
    match t {
        //the possible types
	integer => ~"integer",
	string  => ~"string", 
    }
}
//-----------------------------------------------------------------------------
fn debug_channel<T>(c: @Channel<T>){
	io::println("**************************************");

    io::println(fmt!("%s", debug_name(c.name)));
    //io::println(get_type(c.msg_type));
       
	io::println(fmt!("connections: %u", c.connections.len()));
	let mut count = 0;
	while(count < c.connections.len()){
		io::println(fmt!("- %s", debug_name(c.connections[count].name)));
		count  = count + 1;
	}
	io::println("**************************************");         
}
//-----------------------------------------------------------------------------
fn debug_name(n:Name) -> ~str{
	//io::print(fmt!("%s:%s:%s\n", n.location, n.actor, n.chan_name));
	n.location + ~":" + n.actor + ~":" + n.chan_name
}
//****************************************************************************
//Operations
//****************************************************************************
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
fn create_channel<T>(n:~str, mt:T, d:direction, limit:uint) -> Channel<T>{

	let mut nme = Name{location:~"location", actor:~"actor", chan_name:n};
	Channel{name:nme, msg_type:mt, connections:~[], buffer:~[], buff_limit:limit, dir:d}
}

//-----------------------------------------------------------------------------
fn channel_eq<T>(a: @Channel<T>, b: @Channel<T>) -> bool{
	
	//let i = a.name.location + ~":" + a.name.actor + ~":" + a.name.chan_name;
	//let j = b.name.location + ~":" + b.name.actor + ~":" + b.name.chan_name;
	//io::print(fmt!("channel_eq: %s || %s\n", i,j));
	
	a.name.location == b.name.location && a.name.actor == b.name.actor && a.name.chan_name == b.name.chan_name 
}
//-----------------------------------------------------------------------------
fn channel_connect<T>(a: @Channel<T>, b: @Channel<T>) ->bool {

	if same_direction(a.dir, b.dir) {
		io::print(~"channel_connect: mismatching channel polarity\n");				
		return false;
	}

	//if not already connected
	io::print(fmt!("channel_connect: '%s', has %u connections\n", debug_name(a.name), a.connections.len()));
	
	let mut count = 0; 
	while count < a.connections.len(){
		if channel_eq(a.connections[count], b) {
			io::print(~"channel_connect: channels already connected\n");
			return false
		}
		count += 1;
	}

	vec::push(&mut a.connections, b);
	vec::push(&mut b.connections, a);

	io::print(~"channel_connect: successfully added, do we need to move messages?\n");


	true
}
//----------------------------------------------------------------------------
fn channel_disconnect<T>(a: @Channel<T>, b: @Channel<T>) -> bool{
	
	if a.connections.is_empty() {
		io::print(fmt!("channel_disconnect: '%s', has no connections\n", debug_name(a.name)));
		return false;
	}

	if b.connections.is_empty() {
		io::print(fmt!("channel_disconnect: '%s', has no connections\n", debug_name(a.name)));
		return false;
	}

	let mut count = 0; 
	while count < a.connections.len(){
		if channel_eq(a.connections[count], b) {
			vec::remove(&mut a.connections, count);

			let mut count2 = 0; 
			while count2 < b.connections.len(){
				if channel_eq(b.connections[count], a) {
					vec::remove(&mut b.connections, count2);
				}
			}
			io::print(~"channel_disconnect: success\n");	
			return true;
		}
		count += 1;
	}
	io::print(~"channel_disconnect: error\n");
	false
}
//-----------------------------------------------------------------------------
fn channel_recv<T>(c: @Channel<T>, sync: bool) -> bool{

	if c.connections.is_empty() {
		io::print(fmt!("channel_recv: '%s', has no connections\n", debug_name(c.name)));
	}		

	if c.buffer.len() == 0 {
		io::print(fmt!("channel_recv: no msgs\n"));		
		
		if sync {
			io::print(fmt!("channel_recv: sync mode (wait for a message)\n"));
		}
		else {
			io::print(fmt!("channel_recv: NOT sync mode (return)\n"));
		}
	}
	else {
		io::print(fmt!("channel_recv: we have messages\n"));		
		//lift from buffer and return
	}
	



	true
}
//-----------------------------------------------------------------------------
fn channel_send<T>(c: @Channel<T>, sync: bool, msg: ~T){
	io::print(fmt!("channel_send: '%s'\n", debug_name(c.name)));
	
	if c.connections.is_empty() {
		io::print(fmt!("channel_send: '%s', has no connections\n", debug_name(c.name)));

		if sync {
			io::print(fmt!("channel_send: sync, wait for a connection\n"));                         
		}
		else {
			io::print(fmt!("channel_send: not sync\n"));
			if c.buffer.len() < c.buff_limit {
				io::print(fmt!("channel_send: store message in the buffer\n"));
				vec::push(&mut c.buffer, msg);
			}
			else {	
				io::print(fmt!("channel_send: no buffer space, what do we do? Error?\n"));
			}
		}
	}
	else {
		if sync {
			io::print(fmt!("channel_send: sync, try and push\n"));                         
			let choice = c.connections[rand::random() % c.connections.len()];

			debug_channel(choice);

			if /*choice has buffer_space (and by extension is reacheable)*/{
				io::print(fmt!("channel_send: pushed\n"));
				vec::push(&mut choice.buffer, msg);
				//wake them up
			}
			else {
				//put in our *output* buffer and wait till they pull
				
			}
		}
	}
}
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
//****************************************************************************
//Main
//****************************************************************************

fn main(){
	io::println("Started\n");

	let int_type = 6;
	let mut a = @create_channel(~"a", int_type, in, 10);
	let mut b = @create_channel(~"b", int_type, out, 10);


	debug_channel(a);
	debug_channel(b);


	channel_disconnect(a, b);

	channel_connect(a,b);
	channel_connect(a,b);

	debug_channel(a);
	debug_channel(b);

	//channel_disconnect(a, b);

	channel_recv(a, true);

	let msg = ~6;
	channel_send(a, true, msg)

	

}

