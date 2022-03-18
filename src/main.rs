extern crate colored;
use colored::*;

// const PI:f32 = 3.14;

static USUARIO: &str = "Caio César";
static IDADE: i8 = 19;

fn main() {
	estiliza();
	tudo();
	estiliza();
}

fn estiliza() {
	println!(
		"\n{}\n", "==========================================================================================".yellow().bold().strikethrough()
	);
}
fn quebra() {
	println!("\n");
}

fn entrada() {
	println!("Olá {}, {} novamente.", USUARIO.red().underline(), "bem vindo".green());
}

fn matematica(a: i32, b: i32) -> i32 {
	println!("{} {} e {}", "Cálculo entre os valores".cyan().bold(), a, b);
	println!("{} + {} = {}", a, b, a + b);
	println!("{} - {} = {}", a, b, a - b);
	println!("{} x {} = {}", a, b, a * b);
	println!("{} / {} = {}", a, b, a / b);
	return a - b;
}

fn tabuada(a: u8) {
	println!("{} {}", "Cálculo da tabuada do número".cyan().bold(), a);
	for i in 1..=10 {
		// if i % 2 == 1 {
		// 	continue;
		// }

		println!("{} x {} = {}", a, i, a * i);
	}
}

fn repeticoes() {
	tabuada(5);
}

fn dirigir() {
	let habilitacao = false;
	if IDADE >= 18 && habilitacao {
		println!("{}", "Pode dirigir.".green().bold());
	} else {
		println!("{}", "Não tem permissão para dirigir.".red().bold());
	}
}

fn exercito() {
	let dispensado = true;
	let alistamento = if IDADE >= 18 && !dispensado {
		"Necessário fazer o alistamento.".green().bold()
	} else {
		"Não pode ou não é necessário se alistar.".red().bold()
	};
	println!("{}", alistamento);
}

fn linguagem_proposito(a: &'static str) {
	let linguagem = a;
	let proposito = match linguagem {
		"PHP" => "web",
		"Kotlin" => "android",
		"Python" => "Data Science",
		_ => "uma linguagem desconhecida",
	};

	println!("A linguagem {} é {}.", linguagem.blue().bold(), proposito.blue())
}

fn condicionais() {
	dirigir();
	quebra();
	exercito();
	quebra();
	linguagem_proposito("Rust");
}

fn tudo() {
	entrada();
	quebra();
	matematica(20, 10);
	quebra();
	condicionais();
	quebra();
	repeticoes();
	quebra();
}
