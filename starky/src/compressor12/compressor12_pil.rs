pub fn render(nBits: usize, nPublics: usize) -> String {
    let mut res = String::from("");
    res.push_str(&format!(
        r#"
constant %N = 2**{};

namespace Global(%N);
    pol constant L1;
"#,
        nBits
    ));
    for i in (12..nPublics).step_by(12) {
        res.push_str(&format!(
            r#"
    pol constant L{};
            "#,
            i / 12 + 1
        ));
    }

    res.push_str(&format!(
        r#"
namespace Compressor(%N);
    pol constant S[12];
    pol constant Qm, Ql, Qr, Qo, Qk, QMDS, QCMul;
    pol commit a[12];
            "#
    ));

    for i in 0..nPublics {
        res.push_str(&format!(
            r#"
    public pub{} = a[{}]({});
            "#,
            i,
            i % 12,
            i / 12
        ));
    }

    for i in 0..nPublics {
        res.push_str(&format!(
            r#"
    Global.L{} * (a[{}] - :pub{}) = 0;
            "#,
            i / 12 + 1,
            i % 12,
            i
        ));
    }

    res.push_str(&format!(r#"
// Normal plonk ecuations
    pol a01 = a[0]*a[1];
    Qm*a01 + Ql*a[0] + Qr*a[1] + Qo*a[2] + Qk = 0;

    pol a34 = a[3]*a[4];
    Qm*a34 + Ql*a[3] + Qr*a[4] + Qo*a[5] + Qk = 0;

    pol a67 = a[6]*a[7];
    Qm*a67 + Ql*a[6] + Qr*a[7] + Qo*a[8] +Qk = 0;

    pol a910 = a[9]*a[10];
    Qm*a910 + Ql*a[9] + Qr*a[10] + Qo*a[11] +Qk = 0;

// MDS

    QMDS * (a[ 0]' - (25*a[0] + 15*a[1] + 41*a[2] + 16*a[3] +  2*a[4] + 28*a[5] + 13*a[6] + 13*a[7] + 39*a[8] + 18*a[9] + 34*a[10] + 20*a[11])) = 0;
    QMDS * (a[ 1]' - (20*a[0] + 17*a[1] + 15*a[2] + 41*a[3] + 16*a[4] +  2*a[5] + 28*a[6] + 13*a[7] + 13*a[8] + 39*a[9] + 18*a[10] + 34*a[11])) = 0;
    QMDS * (a[ 2]' - (34*a[0] + 20*a[1] + 17*a[2] + 15*a[3] + 41*a[4] + 16*a[5] +  2*a[6] + 28*a[7] + 13*a[8] + 13*a[9] + 39*a[10] + 18*a[11])) = 0;
    QMDS * (a[ 3]' - (18*a[0] + 34*a[1] + 20*a[2] + 17*a[3] + 15*a[4] + 41*a[5] + 16*a[6] +  2*a[7] + 28*a[8] + 13*a[9] + 13*a[10] + 39*a[11])) = 0;
    QMDS * (a[ 4]' - (39*a[0] + 18*a[1] + 34*a[2] + 20*a[3] + 17*a[4] + 15*a[5] + 41*a[6] + 16*a[7] +  2*a[8] + 28*a[9] + 13*a[10] + 13*a[11])) = 0;
    QMDS * (a[ 5]' - (13*a[0] + 39*a[1] + 18*a[2] + 34*a[3] + 20*a[4] + 17*a[5] + 15*a[6] + 41*a[7] + 16*a[8] +  2*a[9] + 28*a[10] + 13*a[11])) = 0;
    QMDS * (a[ 6]' - (13*a[0] + 13*a[1] + 39*a[2] + 18*a[3] + 34*a[4] + 20*a[5] + 17*a[6] + 15*a[7] + 41*a[8] + 16*a[9] +  2*a[10] + 28*a[11])) = 0;
    QMDS * (a[ 7]' - (28*a[0] + 13*a[1] + 13*a[2] + 39*a[3] + 18*a[4] + 34*a[5] + 20*a[6] + 17*a[7] + 15*a[8] + 41*a[9] + 16*a[10] +  2*a[11])) = 0;
    QMDS * (a[ 8]' - ( 2*a[0] + 28*a[1] + 13*a[2] + 13*a[3] + 39*a[4] + 18*a[5] + 34*a[6] + 20*a[7] + 17*a[8] + 15*a[9] + 41*a[10] + 16*a[11])) = 0;
    QMDS * (a[ 9]' - (16*a[0] +  2*a[1] + 28*a[2] + 13*a[3] + 13*a[4] + 39*a[5] + 18*a[6] + 34*a[7] + 20*a[8] + 17*a[9] + 15*a[10] + 41*a[11])) = 0;
    QMDS * (a[10]' - (41*a[0] + 16*a[1] +  2*a[2] + 28*a[3] + 13*a[4] + 13*a[5] + 39*a[6] + 18*a[7] + 34*a[8] + 20*a[9] + 17*a[10] + 15*a[11])) = 0;
    QMDS * (a[11]' - (15*a[0] + 41*a[1] + 16*a[2] +  2*a[3] + 28*a[4] + 13*a[5] + 13*a[6] + 39*a[7] + 18*a[8] + 34*a[9] + 20*a[10] + 17*a[11])) = 0;

// CMUL

    pol A = (a[0] + a[1])  * (a[3] + a[4]);
    pol B = (a[0] + a[2])  * (a[3] + a[5]);
    pol C = (a[1] + a[2])  * (a[4] + a[5]);
    pol D = a[0]*a[3];
    pol E = a[1]*a[4];
    pol F = a[2]*a[5];

    QCMul * (a[6] - (C + D - E - F)) = 0;
    QCMul * (a[7] - (A + C - 2*E - D)) = 0;
    QCMul * (a[8] - (B - D + E)) = 0;

// Connection equations
    {{a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11]}} connect
        {{S[0], S[1], S[2], S[3], S[4], S[5], S[6], S[7], S[8], S[9], S[10], S[11]}};
"#));
    res
}
