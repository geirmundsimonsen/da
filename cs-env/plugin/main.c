#include "csound/csdl.h"
#include <stdbool.h>

double cubic_function(double x) {
    double x2 = x * x;
    double x3 = x2 * x;
    return 1.00000544855 - 0.5058770015 * x + 0.0938973731 * x2 - 0.0063159426 * x3;
}

typedef struct {
  double* v_start;
  bool v_start_is_audio;
  double* str;
  bool str_is_audio;
  double* cf;
  bool cf_is_audio;
  double* v_end;
  bool v_end_is_audio;
} env_core_args;

typedef struct {
  OPDS h;
  double *out;
  double *v1, *str1, *cf1, *v2, *str2, *cf2, *v3, *str3, *cf3, *v4, *str4, *cf4, *v5;
  double env_v;
  double sr_inv;
  int stage;
  env_core_args core_args;
} env;

void env_update_core_args(CSOUND* csound, env* p, double* v_start, double* str, double* cf, double* v_end) {
  p->core_args.v_start = v_start;
  p->core_args.v_start_is_audio = csound->GetTypeForArg(v_start)->varTypeName == "a";
  p->core_args.str = str;
  p->core_args.str_is_audio = csound->GetTypeForArg(str)->varTypeName == "a";
  p->core_args.cf = cf;
  p->core_args.cf_is_audio = csound->GetTypeForArg(cf)->varTypeName == "a";
  p->core_args.v_end = v_end;
  p->core_args.v_end_is_audio = csound->GetTypeForArg(v_end)->varTypeName == "a";
}

double env_process_core(env* p, int idx) {
  double v_start_v = p->core_args.v_start_is_audio ? p->core_args.v_start[idx] : *p->core_args.v_start;
  double v_end_v = p->core_args.v_end_is_audio ? p->core_args.v_end[idx] : *p->core_args.v_end;
  double str_v = p->core_args.str_is_audio ? p->core_args.str[idx] * p->sr_inv : *p->core_args.str * p->sr_inv;
  double cf_v = p->core_args.cf_is_audio ? p->core_args.cf[idx] : *p->core_args.cf;

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
  
  return p->env_v * (v_end_v - v_start_v) + v_start_v;
}

int env_init(CSOUND* csound, env* p) {
  p->sr_inv = 1.0 / csound->GetSr(csound);
  p->stage = 1;
  env_update_core_args(csound, p, p->v1, p->str1, p->cf1, p->v2);

  return OK;
}

int env1_process(CSOUND* csound, env* p) {
  for (int i = 0; i < CS_KSMPS; i++) {
    if (p->env_v >= 1.0) {
      *p->out = *p->v2;
    } else {
      p->out[i] = env_process_core(p, i);
    }
  }

  return OK;
}

int env2_process(CSOUND* csound, env* p) {
  for (int i = 0; i < CS_KSMPS; i++) {
    if (p->stage > 2) {
      *p->out = *p->v3;
    } else {
      p->out[i] = env_process_core(p, i);
      if (p->env_v >= 1.0) {
        p->stage++;
        p->env_v = 0.0;
        printf("stage: %d\n", p->stage);
        if (p->stage == 2) {
          env_update_core_args(csound, p, p->v2, p->str2, p->cf2, p->v3);
        }
      } 
    }
  }

  return OK;
}

int env3_process(CSOUND* csound, env* p) {
  for (int i = 0; i < CS_KSMPS; i++) {
    if (p->stage > 3) {
      *p->out = *p->v4;
    } else {
      p->out[i] = env_process_core(p, i);
      if (p->env_v >= 1.0) {
        p->stage++;
        p->env_v = 0.0;
        printf("stage: %d\n", p->stage);
        if (p->stage == 2) {
          env_update_core_args(csound, p, p->v2, p->str2, p->cf2, p->v3);
        } else if (p->stage == 3) {
          env_update_core_args(csound, p, p->v3, p->str3, p->cf3, p->v4);
        }
      } 
    }
  }

  return OK;
}

int env4_process(CSOUND* csound, env* p) {
  for (int i = 0; i < CS_KSMPS; i++) {
    if (p->stage > 4) {
      *p->out = *p->v5;
    } else {
      p->out[i] = env_process_core(p, i);
      if (p->env_v >= 1.0) {
        p->stage++;
        p->env_v = 0.0;
        printf("stage: %d\n", p->stage);
        if (p->stage == 2) {
          env_update_core_args(csound, p, p->v2, p->str2, p->cf2, p->v3);
        } else if (p->stage == 3) {
          env_update_core_args(csound, p, p->v3, p->str3, p->cf3, p->v4);
        } else if (p->stage == 4) {
          env_update_core_args(csound, p, p->v4, p->str4, p->cf4, p->v5);
        }
      } 
    }
  }

  return OK;
}

static OENTRY localops[] = {
  { 
    .opname = "env1",
    .dsblksiz = sizeof(env),
    .flags = 0,
    .thread = 3,
    .outypes = "a",
    .intypes = "xxxx",
    .iopadr = (SUBR) env_init,
    .kopadr = (SUBR) env1_process,
  },
  { 
    .opname = "env2",
    .dsblksiz = sizeof(env),
    .flags = 0,
    .thread = 3,
    .outypes = "a",
    .intypes = "xxxxxxx",
    .iopadr = (SUBR) env_init,
    .kopadr = (SUBR) env2_process,
  },
  { 
    .opname = "env3",
    .dsblksiz = sizeof(env),
    .flags = 0,
    .thread = 3,
    .outypes = "a",
    .intypes = "xxxxxxxxxx",
    .iopadr = (SUBR) env_init,
    .kopadr = (SUBR) env3_process,
  },
  {
    .opname = "env4",
    .dsblksiz = sizeof(env),
    .flags = 0,
    .thread = 3,
    .outypes = "a",
    .intypes = "xxxxxxxxxxxxx",
    .iopadr = (SUBR) env_init,
    .kopadr = (SUBR) env4_process,
  },
};

LINKAGE
