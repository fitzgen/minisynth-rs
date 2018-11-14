use super::Result;
use crate::abstract_interpret::{interpret, AbstractInterpret};
use crate::ast::{self, NodeId};
use failure::bail;
use std::collections::{HashMap, HashSet};
use z3;

struct Synthesize<'a, 'ctx>
where
    'ctx: 'a,
{
    ctx: &'ctx z3::Context,
    vars: &'a mut HashMap<String, z3::Ast<'ctx>>,
    holes: &'a mut HashMap<z3::Ast<'ctx>, String>,
    const_vars: &'a mut HashSet<z3::Ast<'ctx>>,
}

impl<'a, 'ctx> AbstractInterpret for Synthesize<'a, 'ctx> {
    type Output = z3::Ast<'ctx>;

    fn constant(&mut self, c: i64) -> z3::Ast<'ctx> {
        z3::Ast::bitvector_from_i64(self.ctx, c as i64, 64)
    }

    fn add(&mut self, lhs: &z3::Ast<'ctx>, rhs: &z3::Ast<'ctx>) -> z3::Ast<'ctx> {
        lhs.bvadd(rhs)
    }

    fn sub(&mut self, lhs: &z3::Ast<'ctx>, rhs: &z3::Ast<'ctx>) -> z3::Ast<'ctx> {
        lhs.bvsub(rhs)
    }

    fn mul(&mut self, lhs: &z3::Ast<'ctx>, rhs: &z3::Ast<'ctx>) -> z3::Ast<'ctx> {
        lhs.bvmul(rhs)
    }

    fn div(&mut self, lhs: &z3::Ast<'ctx>, rhs: &z3::Ast<'ctx>) -> Result<z3::Ast<'ctx>> {
        Ok(lhs.bvsdiv(rhs))
    }

    fn shr(&mut self, lhs: &z3::Ast<'ctx>, rhs: &z3::Ast<'ctx>) -> z3::Ast<'ctx> {
        lhs.bvlshr(&rhs)
    }

    fn shl(&mut self, lhs: &z3::Ast<'ctx>, rhs: &z3::Ast<'ctx>) -> z3::Ast<'ctx> {
        lhs.bvshl(&rhs)
    }

    fn neg(&mut self, e: &z3::Ast<'ctx>) -> z3::Ast<'ctx> {
        e.bvneg()
    }

    fn eq(&mut self, lhs: &z3::Ast<'ctx>, rhs: &z3::Ast<'ctx>) -> z3::Ast<'ctx> {
        lhs._eq(rhs).ite(&self.constant(1), &self.constant(0))
    }

    fn neq(&mut self, lhs: &z3::Ast<'ctx>, rhs: &z3::Ast<'ctx>) -> z3::Ast<'ctx> {
        lhs._eq(rhs).not().ite(&self.constant(1), &self.constant(0))
    }

    fn lookup(&mut self, var: &str) -> Result<z3::Ast<'ctx>> {
        if !self.vars.contains_key(var) {
            let c = self.ctx.fresh_bitvector_const(var, 64);
            self.vars.insert(var.to_string(), c.clone());

            // Insert the variable into either our set of holes, or our set of
            // constants.
            if var.starts_with("h") {
                self.holes.insert(c, var.to_string());
            } else {
                self.const_vars.insert(c);
            }
        }

        Ok(self.vars[var].clone())
    }
}

pub fn synthesize<'a>(
    z3_ctx: &'a z3::Context,
    ast_ctx: &mut ast::Context,
    specification: NodeId,
    template: NodeId,
) -> Result<HashMap<String, i64>> {
    let mut vars = HashMap::new();
    let mut holes = HashMap::new();
    let mut const_vars = HashSet::new();

    let synth = &mut Synthesize {
        ctx: z3_ctx,
        vars: &mut vars,
        holes: &mut holes,
        const_vars: &mut const_vars,
    };

    let specification = interpret(synth, ast_ctx, specification)?;
    if !synth.holes.is_empty() {
        bail!("the specification cannot have any holes!");
    }
    let template = interpret(synth, ast_ctx, template)?;

    let const_vars: Vec<_> = const_vars.iter().collect();
    let templ_eq_spec = specification._eq(&template);
    let goal = if const_vars.is_empty() {
        templ_eq_spec
    } else {
        z3::Ast::forall_const(&const_vars, &templ_eq_spec)
    };

    let solver = z3::Solver::new(z3_ctx);
    solver.assert(&goal);
    if solver.check() {
        let model = solver.get_model();
        let mut results = HashMap::new();
        for (hole, name) in holes {
            results.insert(name, model.eval(&hole).unwrap().as_i64().unwrap());
        }
        Ok(results)
    } else {
        bail!("no solution")
    }
}
