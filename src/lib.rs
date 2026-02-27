use anchor_lang::prelude::*;
declare_id!("");

#[program]
pub mod pasteleria {
    use super::*;

    pub fn crear_pasteleria(context: Context<SetPasteleria>, nombre: String) -> Result<()> {
        let owner = context.accounts.owner.key();
        let pasteles: Vec<Pastel> = Vec::new();

        context.accounts.pasteleria.set_inner(Pasteleria {
            owner,
            nombre,
            pasteles,
        });

        Ok(())
    }

    pub fn agregar_pastel(context: Context<SetPastel>, nombre: String, precio: u16) -> Result<()> {
        let pastel = Pastel {
            nombre,
            precio,
            disponible: true,
        };

        context.accounts.pasteleria.pasteles.push(pastel);

        Ok(())
    }

    pub fn ver_pasteles(context: Context<SetPastel>) -> Result<()> {
        msg!(
            "La lista de pasteles es: {:#?}",
            context.accounts.pasteleria.pasteles
        );

        Ok(())
    }

    pub fn eliminar_pastel(context: Context<SetPastel>, nombre: String) -> Result<()> {
        let pasteles = &mut context.accounts.pasteleria.pasteles;

        for pastel in 0..pasteles.len() {
            if pasteles[pastel].nombre == nombre {
                pasteles.remove(pastel);
                msg!("Pastel {nombre} eliminado!");
                return Ok(());
            }
        }

        Err(Errores::PastelNoExiste.into())
    }

    pub fn alternar_estado(context: Context<SetPastel>, nombre: String) -> Result<()> {
        let pasteles = &mut context.accounts.pasteleria.pasteles;

        for pastel in 0..pasteles.len() {
            let estado = pasteles[pastel].disponible;

            if pasteles[pastel].nombre == nombre {
                let nuevo_estado = !estado;
                pasteles[pastel].disponible = nuevo_estado;

                msg!(
                    "El pastel: {} ahora tiene un valor de disponibilidad: {}",
                    nombre,
                    nuevo_estado
                );
                return Ok(());
            }
        }

        Err(Errores::PastelNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la cuenta.")]
    NoEresElOwner,

    #[msg("Error, el pastel proporcionado no existe.")]
    PastelNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Pasteleria {
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    pasteles: Vec<Pastel>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Pastel {
    #[max_len(60)]
    nombre: String,

    precio: u16,

    disponible: bool,
}

#[derive(Accounts)]
pub struct SetPasteleria<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init, 
        payer = owner, 
        space = Pasteleria::INIT_SPACE + 8, 
        seeds = [b"pasteleria", owner.key().as_ref()], 
        bump
    )]
    pub pasteleria: Account<'info, Pasteleria>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetPastel<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub pasteleria: Account<'info, Pasteleria>,
}
