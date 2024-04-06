use mathml_tokens::from_str;

#[test]
fn elements_to_tokens() {
  let xml = r#"<math>
                  <mrow>
                    <mi>x</mi>
                    <mo>+</mo>
                    <mi>y</mi>
                  </mrow>
                  <mo>=</mo>
                  <mn>1</mn>
               </math>"#;
  let tokens = from_str(xml);
  assert_eq!(tokens, "x + y = 1");
}

#[test]
fn latexml_quadratic_formula_to_tokens() {
  let xml = r###"
<math id="S0.Ex1.m1.1" class="ltx_Math" alttext="x=\frac{-b\pm\sqrt{b^{2}-4ac}}{2a}," display="block"><semantics id="S0.Ex1.m1.1a"><mrow id="S0.Ex1.m1.1.1.1" xref="S0.Ex1.m1.1.1.1.1.cmml"><mrow id="S0.Ex1.m1.1.1.1.1" xref="S0.Ex1.m1.1.1.1.1.cmml"><mi id="S0.Ex1.m1.1.1.1.1.2" xref="S0.Ex1.m1.1.1.1.1.2.cmml">x</mi><mo id="S0.Ex1.m1.1.1.1.1.1" xref="S0.Ex1.m1.1.1.1.1.1.cmml">=</mo><mfrac id="S0.Ex1.m1.1.1.1.1.3" xref="S0.Ex1.m1.1.1.1.1.3.cmml"><mrow id="S0.Ex1.m1.1.1.1.1.3.2" xref="S0.Ex1.m1.1.1.1.1.3.2.cmml"><mrow id="S0.Ex1.m1.1.1.1.1.3.2.2" xref="S0.Ex1.m1.1.1.1.1.3.2.2.cmml"><mo id="S0.Ex1.m1.1.1.1.1.3.2.2a" xref="S0.Ex1.m1.1.1.1.1.3.2.2.cmml">−</mo><mi id="S0.Ex1.m1.1.1.1.1.3.2.2.2" xref="S0.Ex1.m1.1.1.1.1.3.2.2.2.cmml">b</mi></mrow><mo id="S0.Ex1.m1.1.1.1.1.3.2.1" xref="S0.Ex1.m1.1.1.1.1.3.2.1.cmml">±</mo><msqrt id="S0.Ex1.m1.1.1.1.1.3.2.3" xref="S0.Ex1.m1.1.1.1.1.3.2.3.cmml"><mrow id="S0.Ex1.m1.1.1.1.1.3.2.3.2" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.cmml"><msup id="S0.Ex1.m1.1.1.1.1.3.2.3.2.2" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.2.cmml"><mi id="S0.Ex1.m1.1.1.1.1.3.2.3.2.2.2" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.2.2.cmml">b</mi><mn id="S0.Ex1.m1.1.1.1.1.3.2.3.2.2.3" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.2.3.cmml">2</mn></msup><mo id="S0.Ex1.m1.1.1.1.1.3.2.3.2.1" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.1.cmml">−</mo><mrow id="S0.Ex1.m1.1.1.1.1.3.2.3.2.3" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.cmml"><mn id="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.2" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.2.cmml">4</mn><mo id="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.1" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.1.cmml">⁢</mo><mi id="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.3" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.3.cmml">a</mi><mo id="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.1a" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.1.cmml">⁢</mo><mi id="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.4" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.4.cmml">c</mi></mrow></mrow></msqrt></mrow><mrow id="S0.Ex1.m1.1.1.1.1.3.3" xref="S0.Ex1.m1.1.1.1.1.3.3.cmml"><mn id="S0.Ex1.m1.1.1.1.1.3.3.2" xref="S0.Ex1.m1.1.1.1.1.3.3.2.cmml">2</mn><mo id="S0.Ex1.m1.1.1.1.1.3.3.1" xref="S0.Ex1.m1.1.1.1.1.3.3.1.cmml">⁢</mo><mi id="S0.Ex1.m1.1.1.1.1.3.3.3" xref="S0.Ex1.m1.1.1.1.1.3.3.3.cmml">a</mi></mrow></mfrac></mrow><mo id="S0.Ex1.m1.1.1.1.2" xref="S0.Ex1.m1.1.1.1.1.cmml">,</mo></mrow><annotation-xml encoding="MathML-Content" id="S0.Ex1.m1.1b"><apply id="S0.Ex1.m1.1.1.1.1.cmml" xref="S0.Ex1.m1.1.1.1"><eq id="S0.Ex1.m1.1.1.1.1.1.cmml" xref="S0.Ex1.m1.1.1.1.1.1"></eq><ci id="S0.Ex1.m1.1.1.1.1.2.cmml" xref="S0.Ex1.m1.1.1.1.1.2">𝑥</ci><apply id="S0.Ex1.m1.1.1.1.1.3.cmml" xref="S0.Ex1.m1.1.1.1.1.3"><divide id="S0.Ex1.m1.1.1.1.1.3.1.cmml" xref="S0.Ex1.m1.1.1.1.1.3"></divide><apply id="S0.Ex1.m1.1.1.1.1.3.2.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2"><csymbol cd="latexml" id="S0.Ex1.m1.1.1.1.1.3.2.1.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.1">plus-or-minus</csymbol><apply id="S0.Ex1.m1.1.1.1.1.3.2.2.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.2"><minus id="S0.Ex1.m1.1.1.1.1.3.2.2.1.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.2"></minus><ci id="S0.Ex1.m1.1.1.1.1.3.2.2.2.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.2.2">𝑏</ci></apply><apply id="S0.Ex1.m1.1.1.1.1.3.2.3.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.3"><root id="S0.Ex1.m1.1.1.1.1.3.2.3a.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.3"></root><apply id="S0.Ex1.m1.1.1.1.1.3.2.3.2.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2"><minus id="S0.Ex1.m1.1.1.1.1.3.2.3.2.1.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.1"></minus><apply id="S0.Ex1.m1.1.1.1.1.3.2.3.2.2.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.2"><csymbol cd="ambiguous" id="S0.Ex1.m1.1.1.1.1.3.2.3.2.2.1.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.2">superscript</csymbol><ci id="S0.Ex1.m1.1.1.1.1.3.2.3.2.2.2.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.2.2">𝑏</ci><cn type="integer" id="S0.Ex1.m1.1.1.1.1.3.2.3.2.2.3.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.2.3">2</cn></apply><apply id="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.3"><times id="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.1.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.1"></times><cn type="integer" id="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.2.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.2">4</cn><ci id="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.3.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.3">𝑎</ci><ci id="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.4.cmml" xref="S0.Ex1.m1.1.1.1.1.3.2.3.2.3.4">𝑐</ci></apply></apply></apply></apply><apply id="S0.Ex1.m1.1.1.1.1.3.3.cmml" xref="S0.Ex1.m1.1.1.1.1.3.3"><times id="S0.Ex1.m1.1.1.1.1.3.3.1.cmml" xref="S0.Ex1.m1.1.1.1.1.3.3.1"></times><cn type="integer" id="S0.Ex1.m1.1.1.1.1.3.3.2.cmml" xref="S0.Ex1.m1.1.1.1.1.3.3.2">2</cn><ci id="S0.Ex1.m1.1.1.1.1.3.3.3.cmml" xref="S0.Ex1.m1.1.1.1.1.3.3.3">𝑎</ci></apply></apply></apply></annotation-xml><annotation encoding="application/x-tex" id="S0.Ex1.m1.1c">x=\frac{-b\pm\sqrt{b^{2}-4ac}}{2a},</annotation><annotation encoding="application/x-llamapun" id="S0.Ex1.m1.1d">italic_x = divide start_ARG - italic_b ± square-root start_ARG italic_b start_POSTSUPERSCRIPT 2 end_POSTSUPERSCRIPT - 4 italic_a italic_c end_ARG end_ARG start_ARG 2 italic_a end_ARG ,</annotation></semantics></math>"###;
  let tokens = from_str(xml);
  assert_eq!(tokens, "x = [frac] [arg] - b ± [sqrt] [sup] [arg] b [end_arg] [arg] 2 [end_arg] [end_sup] [end_sqrt] [end_arg] [arg] [end_arg] [end_frac]");
}