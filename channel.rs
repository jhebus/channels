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


//This should be replaced by generics
enum Msg_Type{
    integer,
    string,
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
    mut name:Name,
    mut msg_type:Msg_Type,
    mut connections: ~[@Channel<T>],
	mut buffer:~[@T],
	
}
/*-----------------------------------------------------------------------------*/
trait Chan_t{
	fn create() -> Channel;
	fn connect() -> bool;
	fn disconnect() -> bool;
	fn send();
	fn recv();
	fn select();
}

impl Channel : Chan_t {
	fn create() -> Channel{
		
	}
}
/*-----------------------------------------------------------------------------*/
/*-----------------------------------------------------------------------------*/
/*-----------------------------------------------------------------------------*/
/*-----------------------------------------------------------------------------*/
fn create_channel(n:~str, mt:Msg_Type) -> Channel<Msg_Type>{

	let mut nme = Name{location:~"location", actor:~"actor", chan_name:n};
	Channel{name:nme, msg_type:mt, connections:~[]}
}
/*-----------------------------------------------------------------------------*/
fn channel_eq(a: @Channel<Msg_Type>, b: @Channel<Msg_Type>) -> bool{
	/*
	let i = a.name.location + ~":" + a.name.actor + ~":" + a.name.chan_name;
	let j = b.name.location + ~":" + b.name.actor + ~":" + b.name.chan_name;
	io::print(fmt!("channel_eq: %s || %s\n", i,j));
	*/
	a.name.location == b.name.location && a.name.actor == b.name.actor && a.name.chan_name == b.name.chan_name 
}
/*-----------------------------------------------------------------------------*/
fn channel_connect(a: @Channel, b: @Channel) ->bool {
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

	io::print(~"channel_connect: successfully added\n");
	true
}
/*-----------------------------------------------------------------------------*/
fn channel_disconnect(a: @Channel, b: @Channel) -> bool{
	
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
/*-----------------------------------------------------------------------------*/
fn channel_recv(c: @Channel, sync: bool) -> bool{

	if c.connections.is_empty() {
		io::print(fmt!("channel_recv: '%s', has no connections\n", debug_name(c.name)));
		return false;
	}

	/*
	 * Check list
	 * 
	 * Do i have connections
	 */

	true
}
/*-----------------------------------------------------------------------------*/
/*-----------------------------------------------------------------------------*/
/*-----------------------------------------------------------------------------*/
/*-----------------------------------------------------------------------------*/
/*-----------------------------------------------------------------------------*/
/*-----------------------------------------------------------------------------*/
/******************************************************************************/
//DEBUG
/******************************************************************************/
fn get_type(t:Msg_Type) -> ~str {
    match t {
        //the possible types
	integer => ~"integer",
	string  => ~"string", 
    }
}
/*-----------------------------------------------------------------------------*/
fn debug_channel(c: @Channel){
	io::println("**************************************");

    io::println(fmt!("%s", debug_name(c.name)));
    io::println(get_type(c.msg_type));
       
	io::println(fmt!("connections: %u", c.connections.len()));
	let mut count = 0;
	while(count < c.connections.len()){
		io::println(fmt!("- %s", debug_name(c.connections[count].name)));
		count  = count + 1;
	}
	io::println("**************************************");         
}
/*-----------------------------------------------------------------------------*/
fn debug_name(n:Name) -> ~str{
	//io::print(fmt!("%s:%s:%s\n", n.location, n.actor, n.chan_name));
	n.location + ~":" + n.actor + ~":" + n.chan_name
}
/*-----------------------------------------------------------------------------*/
/******************************************************************************/
//Main
/******************************************************************************/
fn main(){
//	let c1:Channel, c2:Channel;

//	let mut a = Channel{name:~"a", msg_type:integer, connections:@[]};
//	let mut b = Channel{name:~"b", msg_type:integer}
	io::println("Started\n");
	
	let mut a = @create_channel(~"a", integer);
	let mut b = @create_channel(~"b", integer);

	debug_channel(a);
	debug_channel(b);


	channel_disconnect(a, b);

	channel_connect(a,b);
	channel_connect(a,b);

	debug_channel(a);
	debug_channel(b);

	channel_disconnect(a, b);

	

}
