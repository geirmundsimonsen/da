#include "csound/csdl.h"

typedef struct _newopc {
  OPDS h;
  double *foo;
  double *bar;
  double counter;
} newopc;

int newopc_init(CSOUND *csound, newopc *p) {
  p->counter = 0;
  //IS_ASIG_ARG(p->bar);
  CS_TYPE* bar_cs_type = csound->GetTypeForArg(p->bar);
  char* bar_var_type_name = bar_cs_type->varTypeName;
  printf("bar_var_type_name: %s\n", bar_var_type_name);
  return OK;
}

static int32_t newopc_process_audio(CSOUND *csound, newopc *p) {
  p->counter += p->bar[64];
  if (p->counter > 1) {
    p->counter = 0;
  }

  
  //int asig_foo = csoundGetTypeForArg(p->foo) == &CS_VAR_TYPE_A;

  if (1) {
    for (int i = 0; i < CS_KSMPS; i++) {
      p->foo[i] = p->counter;
    }
  } else {
    
  }

  return OK;
}

static OENTRY localops[] = {
  { 
    .opname = "newopc",
    .dsblksiz = sizeof(newopc),
    .flags = 0,
    .thread = 3,
    .outypes = "a",
    .intypes = "x",
    .iopadr = (SUBR) newopc_init,
    .kopadr = (SUBR) newopc_process_audio,
  }
};

LINKAGE
