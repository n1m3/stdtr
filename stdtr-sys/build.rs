use cc::Build;

fn main() {
    let files = "acosh.c airy.c asin.c asinh.c atan.c atanh.c bdtr.c beta.c \
                 btdtr.c cbrt.c chbevl.c chdtr.c clog.c cmplx.c const.c \
                 cosh.c dawsn.c drand.c ei.c ellie.c ellik.c ellpe.c ellpj.c ellpk.c \
                 exp.c exp10.c exp2.c expn.c expx2.c fabs.c fac.c fdtr.c \
                 fresnl.c gamma.c gdtr.c hyp2f1.c hyperg.c i0.c i1.c igami.c incbet.c \
                 incbi.c igam.c isnan.c iv.c j0.c j1.c jn.c jv.c k0.c k1.c kn.c kolmogorov.c \
                 log.c log2.c log10.c lrand.c nbdtr.c ndtr.c ndtri.c pdtr.c planck.c \
                 polevl.c polmisc.c polylog.c polyn.c pow.c powi.c psi.c rgamma.c round.c \
                 shichi.c sici.c sin.c sindg.c sinh.c spence.c stdtr.c struve.c \
                 tan.c tandg.c tanh.c unity.c yn.c zeta.c zetac.c \
                 sqrt.c floor.c setprec.c mtherr.c";
    let files = files.split(' ');
    let mut build = Build::new();
    let cephes_dir = "cephes";
    build.include(cephes_dir);
    for file in files {
        let path = format!("{}/{}", cephes_dir, file);
        dbg!(&path);
        build.file(path);
    }
    build.flag_if_supported("-fno-builtin");
    build.warnings(false);
    build.opt_level(2);
    build.compile("cephes");
}
