use anchor_lang::prelude::*;

declare_id!("TvAJdDCghM8UpvhfZQKLmnaKWsN5mq6UYnMQkdChN7y");

#[program]
mod gastochain {
    use super::*;

    pub fn crear_gasto(
        ctx: Context<CrearGasto>,
        monto: u64,
        descripcion: String,
        categoria: String,
    ) -> Result<()> {
        let gasto = &mut ctx.accounts.gasto;
        gasto.monto = monto;
        gasto.descripcion = descripcion;
        gasto.categoria = categoria;

        msg!("Gasto creado correctamente");
        Ok(())
    }

    pub fn leer_gasto(ctx: Context<LeerGasto>) -> Result<()> {
        let gasto = &ctx.accounts.gasto;

        msg!("Monto: {}", gasto.monto);
        msg!("Descripcion: {}", gasto.descripcion);
        msg!("Categoria: {}", gasto.categoria);

        Ok(())
    }

    pub fn actualizar_gasto(
        ctx: Context<ActualizarGasto>,
        monto: u64,
        descripcion: String,
        categoria: String,
    ) -> Result<()> {
        let gasto = &mut ctx.accounts.gasto;
        gasto.monto = monto;
        gasto.descripcion = descripcion;
        gasto.categoria = categoria;

        msg!("Gasto actualizado correctamente");
        Ok(())
    }

    pub fn eliminar_gasto(_ctx: Context<EliminarGasto>) -> Result<()> {
        msg!("Gasto eliminado correctamente");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CrearGasto<'info> {
    #[account(init, payer = signer, space = 8 + Gasto::MAX_SIZE)]
    pub gasto: Account<'info, Gasto>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LeerGasto<'info> {
    pub gasto: Account<'info, Gasto>,
}

#[derive(Accounts)]
pub struct ActualizarGasto<'info> {
    #[account(mut)]
    pub gasto: Account<'info, Gasto>,
}

#[derive(Accounts)]
pub struct EliminarGasto<'info> {
    #[account(mut, close = signer)]
    pub gasto: Account<'info, Gasto>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
pub struct Gasto {
    pub monto: u64,
    pub descripcion: String,
    pub categoria: String,
}

impl Gasto {
    pub const MAX_SIZE: usize = 8 +        // monto
        4 + 100 +  // descripcion
        4 + 50; // categoria
}



