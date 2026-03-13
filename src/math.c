#include "math.h"

#include <gsl/gsl_fit.h>
#include <gsl/gsl_multifit_nlinear.h>

int fit(void)
{
    double x[4] = {1.0, 2.0, 3.0, 4.0};
    double y[4] = {2.2, 3.9, 6.1, 8.0};
    double w[4] = {1.0, 1.0, 1.0, 1.0};
    unsigned int n = 4;
    double c0, c1, cov00, cov01, cov11, chisq;
    gsl_fit_wlinear(x, 1, w, 1, y, 
                    1, n, &c0, &c1, &cov00, &cov01, &cov11, &chisq);
    printf("FIT: y = %gx + %g\n", c1, c0);

    return 0;
}


