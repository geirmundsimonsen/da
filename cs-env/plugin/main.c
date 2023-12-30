#include "csound/csdl.h"

double cubic_function(double x) {
    double x2 = x * x;
    double x3 = x2 * x;
    return 1.00000544855 - 0.5058770015 * x + 0.0938973731 * x2 - 0.0063159426 * x3;
}

typedef struct {
  OPDS h;
  double *out;
  double *v1, *str1, *cf1, *v2;
  double env_v;
  double sr_inv;
} env1;

int env1_init(CSOUND *csound, env1 *p) {
  p->sr_inv = 1.0 / csound->GetSr(csound);

  return OK;
}

int env1_process_audio(CSOUND *csound, env1 *p) {
  double* str_p = p->str1;
  double* cf_p = p->cf1;
  double* v_start_p = p->v1;
  double* v_end_p = p->v2;

  double asig_str = csound->GetTypeForArg(p->str1)->varTypeName == "a";
  double asig_cf = csound->GetTypeForArg(p->cf1)->varTypeName == "a";
  double asig_v_start = csound->GetTypeForArg(p->v1)->varTypeName == "a";
  double asig_v_end = csound->GetTypeForArg(p->v2)->varTypeName == "a";

  for (int i = 0; i < CS_KSMPS; i++) {
    double v_start_v = asig_v_start ? v_start_p[i] : v_start_p[0];
    double v_end_v = asig_v_end ? v_end_p[i] : v_end_p[0];

    if (p->env_v >= 1.0) {
      p->env_v = 1.0;
    } else {
      double str_v = asig_str ? str_p[i] * p->sr_inv : str_p[0] * p->sr_inv;
      double cf_v = asig_cf ? cf_p[i] : cf_p[0];

      if (str_v < 0.0) {
        str_v = 0.0;
      }

      if (cf_v < -4) {
        cf_v = -4;
      } else if (cf_v > 4) {
        cf_v = 4;
      }
      
      if (cf_v >= 0.0) {
        p->env_v += str_v * (p->env_v * cf_v + cubic_function(cf_v));
      } else {
        p->env_v += str_v * ((1.0 - p->env_v) * -cf_v + cubic_function(-cf_v));
      }
    }
    p->out[i] = p->env_v * (v_end_v - v_start_v) + v_start_v;
  }

  return OK;
}

static OENTRY localops[] = {
  { 
    .opname = "env1",
    .dsblksiz = sizeof(env1),
    .flags = 0,
    .thread = 3,
    .outypes = "a",
    .intypes = "xxxx",
    .iopadr = (SUBR) env1_init,
    .kopadr = (SUBR) env1_process_audio,
  }
};

LINKAGE
