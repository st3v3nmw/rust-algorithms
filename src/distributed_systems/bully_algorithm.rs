enum ElectionMsg {
    StartElection,
    OK,
    Coordinator,
}

#[derive(Debug)]
struct Node<'a> {
    process_id: usize,
    coordinator_id: usize,
    election_phase: bool,
    up: bool,
    processes: &'a mut Vec<Node<'a>>,
}
 
impl Node<'_> {
    fn new(process_id: usize, processes: &'static mut Vec<Node<'static>>) -> Self {
        return Node { process_id: process_id, coordinator_id: 0, election_phase: false, up: true, processes: processes };
    }

    fn event(&self, processes: &Vec<Node>) {

    }

    fn send_msg(&self, p_id: usize, msg: ElectionMsg) {
        self.processes[p_id].receive_msg(self.process_id, msg);
    }

    fn receive_msg(&mut self, sender_p_id: usize, msg: ElectionMsg) {
        if self.up {
            match msg {
                ElectionMsg::StartElection => {
                    self.election_phase = true;
                    self.send_msg(sender_p_id, ElectionMsg::OK);
                    self.start_election();
                },
                ElectionMsg::OK => self.election_phase = false,
                ElectionMsg::Coordinator => self.coordinator_id = sender_p_id,
            }
        }
    }

    fn start_election(&self) {
        self.election_phase = true;
        for p_id in self.process_id+1..self.processes.len() { // pick nodes with higher process ids
            self.send_msg(p_id, ElectionMsg::StartElection); // send start election message
        }
    }
}

fn main() {
    let mut processes: Vec<Node> = vec![];
    for i in 1..9 {
        processes.push(Node::new(i as usize, &processes));
    }

    processes[0].start_election();

    for _ in 0..16 {
        for process in &processes {
            process.event(&processes);
        }
    }

    dbg!(&processes);

    a = 5;
    asdf';
}