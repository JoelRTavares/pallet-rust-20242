#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo;
		type MaxLength: Get<u32>;
	}
	use frame_support::BoundedVec;
	use scale_info::prelude::vec::Vec;
    #[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, MaxEncodedLen, Debug)]
    pub struct Filme {
        id: u32,
        nome: BoundedVec<u8, ConstU32<100>>, // Nome limitado a 100 bytes
        bilhetes_vendidos: u32,
        ano_lancamento_entre_2000_e_2025: u32,
        mes_lancamento: u8,
        dia_lancamento: u8,
        genero: Genero,
    }

    #[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, MaxEncodedLen, Debug)]
    pub enum Genero {
        Acao,
        Animacao,
        Comedia,
        Drama,
        Gospel,
        Suspense,
        Outros,
    }


	#[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        FilmeAdicionado { id: u32, nome: Vec<u8> },
		FilmeDisponivel { id: u32, nome: Vec<u8>, bilhetes_vendidos:u32, ano_lancamento_entre_2000_e_2025:u32, mes_lancamento:u8, dia_lancamento:u8, genero:Genero },
		NenhumFilmeArmazenado,
    }

    #[pallet::error]
	pub enum Error<T> {
		NomeMuitoLongo,
		NomeJaExistente,
		FilmeSemNome,
		AnoInvalido,
		MesInvalido,
		DiaInvalido,
		DataInvalida,
		FilmesArmazenamentoCheio,
	}
    #[pallet::storage]
    #[pallet::getter(fn filmes)]
    //pub type Filmes<T> = StorageValue<_, Vec<Filme>, ValueQuery>;
	pub type Filmes<T> = StorageValue<_, BoundedVec<Filme, ConstU32<100>>, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::default())]
        pub fn adicionar_filme(
            origin: OriginFor<T>,
            nome: Vec<u8>,
            bilhetes_vendidos: u32,
            ano_lancamento_entre_2000_e_2025: u32,
            mes_lancamento: u8,
            dia_lancamento: u8,
            genero: Genero,
        ) -> DispatchResult {
            ensure_signed(origin)?;

			

			ensure!(nome.len() > 0, Error::<T>::FilmeSemNome);//Campo filme nao esta em branco
            // Garantir que o nome não exceda 100 bytes
            let nome_bounded: BoundedVec<u8, ConstU32<100>> =
                nome.clone().try_into().map_err(|_| Error::<T>::NomeMuitoLongo)?;

			
			let film = Filmes::<T>::get();
			if film.len() >= 100{
				ensure!(false, Error::<T>::FilmesArmazenamentoCheio);
			}

			for filme in film.iter() {
				if filme.nome == nome_bounded {
					ensure!(false, Error::<T>::NomeJaExistente);
				}
			}

			//Validacoes de data:
			ensure!(ano_lancamento_entre_2000_e_2025 > 1999 && ano_lancamento_entre_2000_e_2025 < 2026, Error::<T>::AnoInvalido);
			ensure!(mes_lancamento > 0 && mes_lancamento < 13, Error::<T>::MesInvalido);
			ensure!(dia_lancamento > 0 && dia_lancamento < 32, Error::<T>::DiaInvalido);

			match mes_lancamento {
                4 | 6 | 9 | 11 if dia_lancamento > 30 => ensure!(false, Error::<T>::DataInvalida),
                2 => {
                    if ano_lancamento_entre_2000_e_2025 % 4 == 0 && (ano_lancamento_entre_2000_e_2025 % 100 != 0 || ano_lancamento_entre_2000_e_2025 % 400 == 0) {
                        if dia_lancamento > 29 {
                            ensure!(false, Error::<T>::DataInvalida);
                        }
                    } else if dia_lancamento > 28{
						ensure!(false, Error::<T>::DataInvalida); 
                    }
                },
				_ => (),
            }

			let mut id = 0;
			if film.len() > 0{
				if let Some(last_index) = film.len().checked_sub(1) {
					if let Some(new_id) = film[last_index].id.checked_add(1) {
						id = new_id;
					}
				}
			}
            let novo_filme = Filme {
                id,
                nome: nome_bounded,
                bilhetes_vendidos,
                ano_lancamento_entre_2000_e_2025,
                mes_lancamento,
                dia_lancamento,
                genero,
            };

			Filmes::<T>::mutate(|filmes| {
				let mut filmes_vec: BoundedVec<Filme, ConstU32<100>> = filmes.clone().into();
				if filmes_vec.try_push(novo_filme).is_ok() {
					*filmes = filmes_vec;
				}
			});
			
            Self::deposit_event(Event::FilmeAdicionado { id, nome });

            Ok(())
        }


        #[pallet::call_index(1)]
        #[pallet::weight(Weight::default())]
        pub fn listar_filmes(origin: OriginFor<T>) -> DispatchResult {
			ensure_signed(origin)?;

			let filmes = Filmes::<T>::get();

			if filmes.is_empty() {
				Self::deposit_event(Event::NenhumFilmeArmazenado);
			} else {
				// Emite um evento com a lista de filmes
				for filme in filmes.iter() {
					Self::deposit_event(Event::FilmeDisponivel {
						id: filme.id,
						nome: filme.nome.to_vec(),
						bilhetes_vendidos: filme.bilhetes_vendidos,
						ano_lancamento_entre_2000_e_2025:filme.ano_lancamento_entre_2000_e_2025,
						mes_lancamento:filme.mes_lancamento,
						dia_lancamento:filme.dia_lancamento,
						genero: filme.genero.clone(),
					});
				}
			}
			Ok(())
        }
    }
}