/*!
Cofactor clearing for G1 and G2.
*/

// Section 7 of https://tools.ietf.org/html/draft-irtf-cfrg-hash-to-curve-04#page-31
// the clear_h method here is to multiply the point $P$ with a scaler $h_ell$
// the $h_ell$ is defined in section 8.7 of the same draft
// where $h_ell = 0xd201000000010001u64$ for G1 and
// $h_ell = 0xbc69f08f2ee75b3584c6a0ea91b352888e2a8e9145ad7689986ff0315
//          08ffe1329c2f178731db956d82bf015d1212b02ec0ec69d7477c1ae954cbc06689
//          f6a359894c0adebbf6b4e8020005aaa95551,$ for G2
// this will be faster than multiplying by the co-factor h

use crate::{
    bls12_381::{G1, G2},
    CurveProjective
};

/* *** addchain for 15132376222941642752 *** */
/* Bos-Coster (win=2) : 69 links, 2 variables */
/// Addition chain implementing exponentiation by -z = 0xd201000000010000
/// input a point p, output p^{-z}
pub(super) fn chain_z<PtT: CurveProjective>(tmpvar1: &mut PtT, tmpvar0: &PtT) {
    *tmpvar1 = *tmpvar0;
    tmpvar1.double(); /*    0 : 2 */
    tmpvar1.add_assign(tmpvar0); /*    1 : 3 */
    for _ in 0..2 {
        tmpvar1.double();
    } /*    2 : 12 */
    tmpvar1.add_assign(tmpvar0); /*    4 : 13 */
    for _ in 0..3 {
        tmpvar1.double();
    } /*    5 : 104 */
    tmpvar1.add_assign(tmpvar0); /*    8 : 105 */
    for _ in 0..9 {
        tmpvar1.double();
    } /*    9 : 53760 */
    tmpvar1.add_assign(tmpvar0); /*   18 : 53761 */
    for _ in 0..32 {
        tmpvar1.double();
    } /*   19 : 230901736800256 */
    tmpvar1.add_assign(tmpvar0); /*   51 : 230901736800257 */
    for _ in 0..16 {
        tmpvar1.double();
    } /*   52 : 15132376222941642752 */
}

// chain for 3 * (z^2 - 1) * h2, used to clear cofactor
// compatibly with Budroni-Pintore GLV-based method
fn chain_h2_eff<PtT: CurveProjective>(tmpvar1: &mut PtT, tmpvar0: &PtT) {
    /* *** addchain for 305502333931268344200999753193121504214466019254188142667664032982267604182971884026507427359259977847832272839041616661285803823378372096355777062779109 *** */
    /* Bos-Coster (win=4) : 604 links, 16 variables */
    *tmpvar1 = *tmpvar0;
    tmpvar1.double(); /*    0 : 2 */
    let mut tmpvar4 = *tmpvar1;
    tmpvar4.add_assign(tmpvar0); /*    1 : 3 */
    let mut tmpvar2 = tmpvar4;
    tmpvar2.add_assign(tmpvar1); /*    2 : 5 */
    let mut tmpvar3 = tmpvar2;
    tmpvar3.add_assign(tmpvar1); /*    3 : 7 */
    let mut tmpvar11 = tmpvar3;
    tmpvar11.add_assign(tmpvar1); /*    4 : 9 */
    let mut tmpvar9 = tmpvar11;
    tmpvar9.add_assign(tmpvar1); /*    5 : 11 */
    let mut tmpvar10 = tmpvar9;
    tmpvar10.add_assign(tmpvar1); /*    6 : 13 */
    let mut tmpvar5 = tmpvar10;
    tmpvar5.add_assign(tmpvar1); /*    7 : 15 */
    let mut tmpvar7 = tmpvar5;
    tmpvar7.add_assign(tmpvar1); /*    8 : 17 */
    let mut tmpvar15 = tmpvar7;
    tmpvar15.add_assign(tmpvar1); /*    9 : 19 */
    let mut tmpvar13 = tmpvar15;
    tmpvar13.add_assign(tmpvar1); /*   10 : 21 */
    let mut tmpvar6 = tmpvar13;
    tmpvar6.add_assign(tmpvar1); /*   11 : 23 */
    let mut tmpvar14 = tmpvar6;
    tmpvar14.add_assign(tmpvar1); /*   12 : 25 */
    let mut tmpvar12 = tmpvar14;
    tmpvar12.add_assign(tmpvar1); /*   13 : 27 */
    let mut tmpvar8 = tmpvar12;
    tmpvar8.add_assign(tmpvar1); /*   14 : 29 */
    *tmpvar1 = tmpvar6;
    tmpvar1.double(); /*   15 : 46 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*   16 : 1472 */
    tmpvar1.add_assign(&tmpvar13); /*   21 : 1493 */
    for _ in 0..2 {
        tmpvar1.double();
    } /*   22 : 5972 */
    tmpvar1.add_assign(tmpvar0); /*   24 : 5973 */
    for _ in 0..9 {
        tmpvar1.double();
    } /*   25 : 3058176 */
    tmpvar1.add_assign(&tmpvar8); /*   34 : 3058205 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*   35 : 97862560 */
    tmpvar1.add_assign(&tmpvar11); /*   40 : 97862569 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*   41 : 6263204416 */
    tmpvar1.add_assign(&tmpvar13); /*   47 : 6263204437 */
    for _ in 0..8 {
        tmpvar1.double();
    } /*   48 : 1603380335872 */
    tmpvar1.add_assign(&tmpvar2); /*   56 : 1603380335877 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*   57 : 51308170748064 */
    tmpvar1.add_assign(&tmpvar3); /*   62 : 51308170748071 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*   63 : 1641861463938272 */
    tmpvar1.add_assign(&tmpvar3); /*   68 : 1641861463938279 */
    for _ in 0..4 {
        tmpvar1.double();
    } /*   69 : 26269783423012464 */
    tmpvar1.add_assign(&tmpvar5); /*   73 : 26269783423012479 */
    for _ in 0..4 {
        tmpvar1.double();
    } /*   74 : 420316534768199664 */
    tmpvar1.add_assign(tmpvar0); /*   78 : 420316534768199665 */
    for _ in 0..8 {
        tmpvar1.double();
    } /*   79 : 107601032900659114240 */
    tmpvar1.add_assign(&tmpvar11); /*   87 : 107601032900659114249 */
    for _ in 0..8 {
        tmpvar1.double();
    } /*   88 : 27545864422568733247744 */
    tmpvar1.add_assign(&tmpvar8); /*   96 : 27545864422568733247773 */
    for _ in 0..4 {
        tmpvar1.double();
    } /*   97 : 440733830761099731964368 */
    tmpvar1.add_assign(&tmpvar2); /*  101 : 440733830761099731964373 */
    for _ in 0..9 {
        tmpvar1.double();
    } /*  102 : 225655721349683062765758976 */
    tmpvar1.add_assign(&tmpvar5); /*  111 : 225655721349683062765758991 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  112 : 14441966166379716017008575424 */
    tmpvar1.add_assign(&tmpvar11); /*  118 : 14441966166379716017008575433 */
    for _ in 0..2 {
        tmpvar1.double();
    } /*  119 : 57767864665518864068034301732 */
    tmpvar1.add_assign(tmpvar0); /*  121 : 57767864665518864068034301733 */
    for _ in 0..9 {
        tmpvar1.double();
    } /*  122 : 29577146708745658402833562487296 */
    tmpvar1.add_assign(&tmpvar8); /*  131 : 29577146708745658402833562487325 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*  132 : 946468694679861068890673999594400 */
    tmpvar1.add_assign(&tmpvar13); /*  137 : 946468694679861068890673999594421 */
    for _ in 0..4 {
        tmpvar1.double();
    } /*  138 : 15143499114877777102250783993510736 */
    tmpvar1.add_assign(tmpvar0); /*  142 : 15143499114877777102250783993510737 */
    for _ in 0..11 {
        tmpvar1.double();
    } /*  143 : 31013886187269687505409605618709989376 */
    tmpvar1.add_assign(&tmpvar9); /*  154 : 31013886187269687505409605618709989387 */
    for _ in 0..7 {
        tmpvar1.double();
    } /*  155 : 3969777431970520000692429519194878641536 */
    tmpvar1.add_assign(&tmpvar12); /*  162 : 3969777431970520000692429519194878641563 */
    for _ in 0..7 {
        tmpvar1.double();
    } /*  163 : 508131511292226560088630978456944466120064 */
    tmpvar1.add_assign(&tmpvar7); /*  170 : 508131511292226560088630978456944466120081 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*  171 : 16260208361351249922836191310622222915842592 */
    tmpvar1.add_assign(&tmpvar12); /*  176 : 16260208361351249922836191310622222915842619 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*  177 : 520326667563239997530758121939911133306963808 */
    tmpvar1.add_assign(&tmpvar14); /*  182 : 520326667563239997530758121939911133306963833 */
    for _ in 0..8 {
        tmpvar1.double();
    } /*  183 : 133203626896189439367874079216617250126582741248 */
    tmpvar1.add_assign(&tmpvar13); /*  191 : 133203626896189439367874079216617250126582741269 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  192 : 8525032121356124119543941069863504008101295441216 */
    tmpvar1.add_assign(&tmpvar3); /*  198 : 8525032121356124119543941069863504008101295441223 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*  199 : 272801027883395971825406114235632128259241454119136 */
    tmpvar1.add_assign(tmpvar0); /*  204 : 272801027883395971825406114235632128259241454119137 */
    for _ in 0..8 {
        tmpvar1.double();
    } /*  205 : 69837063138149368787303965244321824834365812254499072 */
    tmpvar1.add_assign(&tmpvar9); /*  213 : 69837063138149368787303965244321824834365812254499083 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  214 : 4469572040841559602387453775636596789399411984287941312 */
    tmpvar1.add_assign(&tmpvar13); /*  220 : 4469572040841559602387453775636596789399411984287941333 */
    for _ in 0..4 {
        tmpvar1.double();
    } /*  221 : 71513152653464953638199260410185548630390591748607061328 */
    tmpvar1.add_assign(&tmpvar10); /*  225 : 71513152653464953638199260410185548630390591748607061341 */
    for _ in 0..4 {
        tmpvar1.double();
    } /*  226 : 1144210442455439258211188166562968778086249467977712981456 */
    tmpvar1.add_assign(&tmpvar2); /*  230 : 1144210442455439258211188166562968778086249467977712981461 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  231 : 73229468317148112525516042660030001797519965950573630813504 */
    tmpvar1.add_assign(&tmpvar10); /*  237 : 73229468317148112525516042660030001797519965950573630813517 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  238 : 4686685972297479201633026730241920115041277820836712372065088 */
    tmpvar1.add_assign(&tmpvar2); /*  244 : 4686685972297479201633026730241920115041277820836712372065093 */
    for _ in 0..4 {
        tmpvar1.double();
    } /*  245 : 74986975556759667226128427683870721840660445133387397953041488 */
    tmpvar1.add_assign(tmpvar0); /*  249 : 74986975556759667226128427683870721840660445133387397953041489 */
    for _ in 0..10 {
        tmpvar1.double();
    } /*  250 : 76786662970121899239555509948283619164836295816588695503914484736 */
    tmpvar1.add_assign(&tmpvar9); /*  260 : 76786662970121899239555509948283619164836295816588695503914484747 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  261 : 4914346430087801551331552636690151626549522932261676512250527023808 */
    tmpvar1.add_assign(&tmpvar14); /*  267 : 4914346430087801551331552636690151626549522932261676512250527023833 */
    for _ in 0..4 {
        tmpvar1.double();
    } /*  268 : 78629542881404824821304842187042426024792366916186824196008432381328 */
    tmpvar1.add_assign(&tmpvar3); /*  272 : 78629542881404824821304842187042426024792366916186824196008432381335 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  273 : 5032290744409908788563509899970715265586711482635956748544539672405440 */
    tmpvar1.add_assign(&tmpvar9); /*  279 : 5032290744409908788563509899970715265586711482635956748544539672405451 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  280 : 322066607642234162468064633598125776997549534888701231906850539033948864 */
    tmpvar1.add_assign(&tmpvar15); /*  286 : 322066607642234162468064633598125776997549534888701231906850539033948883 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*  287 : 10306131444551493198978068275140024863921585116438439421019217249086364256 */
    tmpvar1.add_assign(&tmpvar8); /*  292 : 10306131444551493198978068275140024863921585116438439421019217249086364285 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*  293 : 329796206225647782367298184804480795645490723726030061472614951970763657120 */
    tmpvar1.add_assign(&tmpvar12); /*  298 : 329796206225647782367298184804480795645490723726030061472614951970763657147 */
    for _ in 0..4 {
        tmpvar1.double();
    } /*  299 : 5276739299610364517876770956871692730327851579616480983561839231532218514352 */
    tmpvar1.add_assign(&tmpvar5); /*  303 : 5276739299610364517876770956871692730327851579616480983561839231532218514367 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  304 : 337711315175063329144113341239788334740982501095454782947957710818061984919488 */
    tmpvar1.add_assign(&tmpvar15); /*  310 : 337711315175063329144113341239788334740982501095454782947957710818061984919507 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  311 : 21613524171204053065223253839346453423422880070109106108669293492355967034848448 */
    tmpvar1.add_assign(&tmpvar2); /*  317 : 21613524171204053065223253839346453423422880070109106108669293492355967034848453 */
    for _ in 0..7 {
        tmpvar1.double();
    } /*  318 : 2766531093914118792348576491436346038198128648973965581909669567021563780460601984 */
    tmpvar1.add_assign(&tmpvar5); /*  325 : 2766531093914118792348576491436346038198128648973965581909669567021563780460601999 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  326 : 177057990010503602710308895451926146444680233534333797242218852289380081949478527936 */
    tmpvar1.add_assign(&tmpvar3); /*  332 : 177057990010503602710308895451926146444680233534333797242218852289380081949478527943 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  333 : 11331711360672230573459769308923273372459534946197363023502006546520325244766625788352 */
    tmpvar1.add_assign(&tmpvar9); /*  339 : 11331711360672230573459769308923273372459534946197363023502006546520325244766625788363 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  340 : 725229527083022756701425235771089495837410236556631233504128418977300815665064050455232 */
    tmpvar1.add_assign(&tmpvar15); /*  346 : 725229527083022756701425235771089495837410236556631233504128418977300815665064050455251 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  347 : 46414689733313456428891215089349727733594255139624398944264218814547252202564099229136064 */
    tmpvar1.add_assign(&tmpvar14); /*  353 : 46414689733313456428891215089349727733594255139624398944264218814547252202564099229136089 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*  354 : 1485270071466030605724518882859191287475016164467980766216455002065512070482051175332354848 */
    tmpvar1.add_assign(&tmpvar8); /*  359 : 1485270071466030605724518882859191287475016164467980766216455002065512070482051175332354877 */
    for _ in 0..10 {
        tmpvar1.double();
    } /*  360 : 1520916553181215340261907336047811878374416552415212304605649922115084360173620403540331394048 */
    tmpvar1.add_assign(&tmpvar6); /*  370 : 1520916553181215340261907336047811878374416552415212304605649922115084360173620403540331394071 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*  371 : 48669329701798890888381034753529980107981329677286793747380797507682699525555852913290604610272 */
    tmpvar1.add_assign(&tmpvar5); /*  376 : 48669329701798890888381034753529980107981329677286793747380797507682699525555852913290604610287 */
    for _ in 0..3 {
        tmpvar1.double();
    } /*  377 : 389354637614391127107048278028239840863850637418294349979046380061461596204446823306324836882296 */
    tmpvar1.add_assign(tmpvar0); /*  380 : 389354637614391127107048278028239840863850637418294349979046380061461596204446823306324836882297 */
    for _ in 0..9 {
        tmpvar1.double();
    } /*  381 : 199349574458568257078808718350458798522291526358166707189271746591468337256676773532838316483736064 */
    tmpvar1.add_assign(&tmpvar13); /*  390 : 199349574458568257078808718350458798522291526358166707189271746591468337256676773532838316483736085 */
    for _ in 0..7 {
        tmpvar1.double();
    } /*  391 : 25516745530696736906087515948858726210853315373845338520226783563707947168854627012203304509918218880 */
    tmpvar1.add_assign(&tmpvar12); /*  398 : 25516745530696736906087515948858726210853315373845338520226783563707947168854627012203304509918218907 */
    for _ in 0..4 {
        tmpvar1.double();
    } /*  399 : 408267928491147790497400255181739619373653045981525416323628537019327154701674032195252872158691502512 */
    tmpvar1.add_assign(&tmpvar5); /*  403 : 408267928491147790497400255181739619373653045981525416323628537019327154701674032195252872158691502527 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  404 : 26129147423433458591833616331631335639913794942817626644712226369236937900907138060496183818156256161728 */
    tmpvar1.add_assign(&tmpvar2); /*  410 : 26129147423433458591833616331631335639913794942817626644712226369236937900907138060496183818156256161733 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  411 : 1672265435099741349877351445224405480954482876340328105261582487631164025658056835871755764362000394350912 */
    tmpvar1.add_assign(&tmpvar11); /*  417 : 1672265435099741349877351445224405480954482876340328105261582487631164025658056835871755764362000394350921 */
    for _ in 0..4 {
        tmpvar1.double();
    } /*  418 : 26756246961595861598037623123590487695271726021445249684185319802098624410528909373948092229792006309614736 */
    tmpvar1.add_assign(&tmpvar10); /*  422 : 26756246961595861598037623123590487695271726021445249684185319802098624410528909373948092229792006309614749 */
    for _ in 0..4 {
        tmpvar1.double();
    } /*  423 : 428099951385533785568601969977447803124347616343123994946965116833577990568462549983169475676672100953835984 */
    tmpvar1.add_assign(&tmpvar4); /*  427 : 428099951385533785568601969977447803124347616343123994946965116833577990568462549983169475676672100953835987 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  428 : 27398396888674162276390526078556659399958247445959935676605767477348991396381603198922846443307014461045503168 */
    tmpvar1.add_assign(&tmpvar10); /*  434 : 27398396888674162276390526078556659399958247445959935676605767477348991396381603198922846443307014461045503181 */
    for _ in 0..7 {
        tmpvar1.double();
    } /*  435 : 3506994801750292771377987338055252403194655673082871766605538237100670898736845209462124344743297851013824407168 */
    tmpvar1.add_assign(&tmpvar7); /*  442 : 3506994801750292771377987338055252403194655673082871766605538237100670898736845209462124344743297851013824407185 */
    for _ in 0..3 {
        tmpvar1.double();
    } /*  443 : 28055958414002342171023898704442019225557245384662974132844305896805367189894761675696994757946382808110595257480 */
    tmpvar1.add_assign(&tmpvar2); /*  446 : 28055958414002342171023898704442019225557245384662974132844305896805367189894761675696994757946382808110595257485 */
    for _ in 0..4 {
        tmpvar1.double();
    } /*  447 : 448895334624037474736382379271072307608915926154607586125508894348885875038316186811151916127142124929769524119760 */
    tmpvar1.add_assign(&tmpvar3); /*  451 : 448895334624037474736382379271072307608915926154607586125508894348885875038316186811151916127142124929769524119767 */
    for _ in 0..8 {
        tmpvar1.double();
    } /*  452 : 114917205663753593532513889093394510747882477095579542048130276953314784009808943823654890528548383982020998174660352 */
    tmpvar1.add_assign(&tmpvar9); /*  460 : 114917205663753593532513889093394510747882477095579542048130276953314784009808943823654890528548383982020998174660363 */
    for _ in 0..8 {
        tmpvar1.double();
    } /*  461 : 29418804649920919944323555607908994751457914136468362764321350900048584706511089618855651975308386299397375532713052928 */
    tmpvar1.add_assign(&tmpvar9); /*  469 : 29418804649920919944323555607908994751457914136468362764321350900048584706511089618855651975308386299397375532713052939 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  470 : 1882803497594938876436707558906175664093306504733975216916566457603109421216709735606761726419736723161432034093635388096 */
    tmpvar1.add_assign(&tmpvar8); /*  476 : 1882803497594938876436707558906175664093306504733975216916566457603109421216709735606761726419736723161432034093635388125 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*  477 : 60249711923038044045974641884997621250985808151487206941330126643299501478934711539416375245431575141165825090996332420000 */
    tmpvar1.add_assign(&tmpvar7); /*  482 : 60249711923038044045974641884997621250985808151487206941330126643299501478934711539416375245431575141165825090996332420017 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*  483 : 1927990781537217409471188540319923880031545860847590622122564052585584047325910769261324007853810404517306402911882637440544 */
    tmpvar1.add_assign(&tmpvar6); /*  488 : 1927990781537217409471188540319923880031545860847590622122564052585584047325910769261324007853810404517306402911882637440567 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  489 : 123391410018381914206156066580475128322018935094245799815844099365477379028858289232724736502643865889107609786360488796196288 */
    tmpvar1.add_assign(&tmpvar5); /*  495 : 123391410018381914206156066580475128322018935094245799815844099365477379028858289232724736502643865889107609786360488796196303 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  496 : 7897050241176442509193988261150408212609211846031731188214022359390552257846930510894383136169207416902887026327071282956563392 */
    tmpvar1.add_assign(&tmpvar4); /*  502 : 7897050241176442509193988261150408212609211846031731188214022359390552257846930510894383136169207416902887026327071282956563395 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*  503 : 252705607717646160294207624356813062803494779073015398022848715500497672251101776348620260357414637340892384842466281054610028640 */
    tmpvar1.add_assign(&tmpvar5); /*  508 : 252705607717646160294207624356813062803494779073015398022848715500497672251101776348620260357414637340892384842466281054610028655 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  509 : 16173158893929354258829287958836036019423665860672985473462317792031851024070513686311696662874536789817112629917841987495041833920 */
    tmpvar1.add_assign(&tmpvar4); /*  515 : 16173158893929354258829287958836036019423665860672985473462317792031851024070513686311696662874536789817112629917841987495041833923 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  516 : 1035082169211478672565074429365506305243114615083071070301588338690038465540512875923948586423970354548295208314741887199682677371072 */
    tmpvar1.add_assign(&tmpvar3); /*  522 : 1035082169211478672565074429365506305243114615083071070301588338690038465540512875923948586423970354548295208314741887199682677371079 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  523 : 66245258829534635044164763479392403535559335365316548499301653676162461794592824059132709531134102691090893332143480780779691351749056 */
    tmpvar1.add_assign(&tmpvar4); /*  529 : 66245258829534635044164763479392403535559335365316548499301653676162461794592824059132709531134102691090893332143480780779691351749059 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  530 : 4239696565090216642826544862681113826275797463380259103955305835274397554853940739784493409992582572229817173257182769969900246511939776 */
    tmpvar1.add_assign(&tmpvar5); /*  536 : 4239696565090216642826544862681113826275797463380259103955305835274397554853940739784493409992582572229817173257182769969900246511939791 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  537 : 271340580165773865140898871211591284881651037656336582653139573457561443510652207346207578239525284622708299088459697278073615776764146624 */
    tmpvar1.add_assign(&tmpvar3); /*  543 : 271340580165773865140898871211591284881651037656336582653139573457561443510652207346207578239525284622708299088459697278073615776764146631 */
    for _ in 0..7 {
        tmpvar1.double();
    } /*  544 : 34731594261219054738035055515083684464851332820011082579601865402567864769363482540314570014659236431706662283322841251593422819425810768768 */
    tmpvar1.add_assign(&tmpvar3); /*  551 : 34731594261219054738035055515083684464851332820011082579601865402567864769363482540314570014659236431706662283322841251593422819425810768775 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  552 : 2222822032718019503234243552965355805750485300480709285094519385764343345239262882580132480938191131629226386132661840101979060443251889201600 */
    tmpvar1.add_assign(&tmpvar3); /*  558 : 2222822032718019503234243552965355805750485300480709285094519385764343345239262882580132480938191131629226386132661840101979060443251889201607 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*  559 : 71130305046976624103495793694891385784015529615382697123024620344458987047656412242564239390022116212135244356245178883263329934184060454451424 */
    tmpvar1.add_assign(&tmpvar4); /*  564 : 71130305046976624103495793694891385784015529615382697123024620344458987047656412242564239390022116212135244356245178883263329934184060454451427 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  565 : 4552339523006503942623730796473048690176993895384492615873575702045375171050010383524111320961415437576655638799691448528853115787779869084891328 */
    tmpvar1.add_assign(&tmpvar3); /*  571 : 4552339523006503942623730796473048690176993895384492615873575702045375171050010383524111320961415437576655638799691448528853115787779869084891335 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  572 : 291349729472416252327918770974275116171327609304607527415908844930904010947200664545543124541530588004905960883180252705846599410417911621433045440 */
    tmpvar1.add_assign(&tmpvar3); /*  578 : 291349729472416252327918770974275116171327609304607527415908844930904010947200664545543124541530588004905960883180252705846599410417911621433045447 */
    for _ in 0..3 {
        tmpvar1.double();
    } /*  579 : 2330797835779330018623350167794200929370620874436860219327270759447232087577605316364344996332244704039247687065442021646772795283343292971464363576 */
    tmpvar1.add_assign(tmpvar0); /*  582 : 2330797835779330018623350167794200929370620874436860219327270759447232087577605316364344996332244704039247687065442021646772795283343292971464363577 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  583 : 149171061489877121191894410738828859479719735963959054036945328604622853604966740247318079765263661058511851972188289385393458898133970750173719268928 */
    tmpvar1.add_assign(&tmpvar3); /*  589 : 149171061489877121191894410738828859479719735963959054036945328604622853604966740247318079765263661058511851972188289385393458898133970750173719268935 */
    for _ in 0..6 {
        tmpvar1.double();
    } /*  590 : 9546947935352135756281242287285047006702063101693379458364501030695862630717871375828357104976874307744758526220050520665181369480574128011118033211840 */
    tmpvar1.add_assign(&tmpvar3); /*  596 : 9546947935352135756281242287285047006702063101693379458364501030695862630717871375828357104976874307744758526220050520665181369480574128011118033211847 */
    for _ in 0..5 {
        tmpvar1.double();
    } /*  597 : 305502333931268344200999753193121504214466019254188142667664032982267604182971884026507427359259977847832272839041616661285803823378372096355777062779104 */
    tmpvar1.add_assign(&tmpvar2); /*  602 : 305502333931268344200999753193121504214466019254188142667664032982267604182971884026507427359259977847832272839041616661285803823378372096355777062779109 */

    // compute 3 * h2
    tmpvar2 = *tmpvar1;
    tmpvar2.double(); // 2 * h2
    tmpvar2.add_assign(tmpvar1); // 3 * h2

    // compute (z**2 - 1) * 3 * h2
    chain_z(&mut tmpvar3, &tmpvar2); // 3 * z * h2
    chain_z(tmpvar1, &tmpvar3); // 3 * z**2 * h2
    tmpvar1.sub_assign(&tmpvar2);
}

/// Trait implementing cofactor clearing for projective coords
pub trait ClearH: CurveProjective {
    /// Clear the cofactor in-place
    fn clear_h(&mut self);
}

impl ClearH for G1 {
    // h_eff = 1 - z, therefore
    // out = in * chain_z(in)
    fn clear_h(&mut self) {
        let pt_in = *self;
        chain_z(self, &pt_in);
        self.add_assign(&pt_in);
    }
}

impl ClearH for G2 {
    // this function emulates the Budroni-Pintore method given by
    // section 4.1, equation 12 of https://eprint.iacr.org/2017/419.pdf
    //
    // To avoid any potential issue with the GLV patent (US 7110538),
    // we *do not* use the endomorphism. Instead, we emulate this method
    // with a scalar multiplication by h2_eff = h2 * (3 * z^2 - 3), where
    // h2 is the cofactor on E2 and z is the BLS parameter, 0xd201000000010000.
    // This is equivalent per [BP17] Section 4.1.
    fn clear_h(&mut self) {
        let pt_in = *self;
        chain_h2_eff(self, &pt_in);
    }
}

/// Tests for cofactor clearing
#[cfg(test)]
mod tests {
    use super::ClearH;
    use crate::bls12_381::{Fq, Fq2, FqRepr, FrRepr, G1, G2};
    use ff::PrimeField;
    //    use rand::{thread_rng, Rand};
    use rand_core::SeedableRng;
    use crate::CurveProjective;
    #[test]
    fn test_clear_h() {
        let mut rng = rand_xorshift::XorShiftRng::from_seed([
            0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
            0xbc, 0xe5,
        ]);
        for _ in 0..32 {
            let mut input = G1::random(&mut rng);
            let mut result = input;
            result.clear_h();
            input.mul_assign(0xd201000000010001u64);
            assert_eq!(result, input);
        }
    }

    #[test]
    fn test_clear_h2() {
        let mut rng = rand_xorshift::XorShiftRng::from_seed([
            0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
            0xbc, 0xe5,
        ]);

        // kinda sorta test
        for _ in 0..32 {
            let mut input = G2::random(&mut rng);
            let mut result = input;
            result.clear_h();
            input.mul_assign(FrRepr([
                0xa40200040001ffffu64,
                0xb116900400069009u64,
                0x0000000000000002u64,
                0x0000000000000000u64,
            ]));
            assert_eq!(result, input);
        }

        // really for real test
        let c0 = Fq::from_repr(FqRepr([
            0x60bfbecc732ba610u64,
            0x6603a5f54c58db2fu64,
            0x5d8eb4297c4d8279u64,
            0xb1bbb083d0728d9du64,
            0x79e52f9d6301e7a9u64,
            0x0c9fb76d56733b44u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x2058eebaac3db022u64,
            0xd8f94159af393618u64,
            0x4e041f53ff779974u64,
            0x03a5f678559fecdcu64,
            0xcdb85eca3da1f440u64,
            0x006d55d738a89daau64,
        ]))
        .unwrap();
        let xi = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0x938225a3e8d53daau64,
            0x30ea7f357aaa77dfu64,
            0x63587f338dc75610u64,
            0x7b35c727ac61e96bu64,
            0x1e003da1f3a124f4u64,
            0x087785cfcb421f1fu64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xa75d1b64c7f88282u64,
            0xdfe0c7eba1fe426eu64,
            0x19272d81b8edef80u64,
            0x9ab5ce196e06fe79u64,
            0x8a355846ccb413d1u64,
            0x0923471c6b752c75u64,
        ]))
        .unwrap();
        let yi = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0x55165d667c9f9812u64,
            0xac6431be755ad550u64,
            0x97c399a16cf5d66bu64,
            0xc4f2c5ff5e7563e7u64,
            0xc240476aa653e0b2u64,
            0x0f7f362adfa23764u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xb6267378d94c97b8u64,
            0x01bc8e83b89eccffu64,
            0x125c9b39aba71843u64,
            0xc130ce1872e2f21au64,
            0xe981bb12aaf40da3u64,
            0x13c645cc354af99du64,
        ]))
        .unwrap();
        let zi = Fq2 { c0, c1 };
        let mut pi = G2 {
            x: xi,
            y: yi,
            z: zi,
        };
        pi.clear_h();
        let c0 = Fq::from_repr(FqRepr([
            0x2a31a2dd0fdb0639u64,
            0x56c20026fc05a72du64,
            0x803739ef1dfbb449u64,
            0x04fc1b828144bdf6u64,
            0xeaceed987948436du64,
            0x1470136456244901u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xa659d96591a5b1ddu64,
            0xe0865f2fb7c23ef2u64,
            0x0ef5af32f3c9d18eu64,
            0x84bd02cb19fc81cfu64,
            0x6b4b92771dd8b717u64,
            0x0b55195ae0adcc28u64,
        ]))
        .unwrap();
        let xo = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xe96d68bfe2d19c9eu64,
            0xc866562b27937ae3u64,
            0xfdf2fc54562635e0u64,
            0x912e94ab3c21d229u64,
            0xc11f34aefe94c01au64,
            0x17c43b238fba8709u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xaaecf08cd1008aa4u64,
            0x6a2f4b8cd343c879u64,
            0x359faf89d61a09a1u64,
            0xa5b3631b436b673bu64,
            0xf8feb650d6b3f3e9u64,
            0x009b1ff5dfcde663u64,
        ]))
        .unwrap();
        let yo = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xec7dce5a7896e240u64,
            0x938083998c2a5d40u64,
            0x39bf9d8500c9c8efu64,
            0xc0bb723e4646e48fu64,
            0xa33859cef4f3d803u64,
            0x16046ed5637f1cebu64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x7fd2dd34eb3df4dbu64,
            0x28ca7b0791108e03u64,
            0x67e02cd3f84a6f33u64,
            0x53e182e58667e803u64,
            0x4bc9e4801c0e6f45u64,
            0x11b7c228955190f9u64,
        ]))
        .unwrap();
        let zo = Fq2 { c0, c1 };
        let po = G2 {
            x: xo,
            y: yo,
            z: zo,
        };
        assert_eq!(pi, po);

        let c0 = Fq::from_repr(FqRepr([
            0x6fa65f65fa648214u64,
            0x2dd4f6998a8cbad5u64,
            0x279f4d81f93074e9u64,
            0x59771054bff8e5c9u64,
            0x301cacbeb813b681u64,
            0x0936c756f4e4ef7au64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x4dbcb567de5d656bu64,
            0x81115c11f506f4a2u64,
            0x9c85b49117e4cd56u64,
            0x9060f0e2b1a73fe1u64,
            0xc83a89a675fd5bf1u64,
            0x0e1d5f9cd7fbe4d8u64,
        ]))
        .unwrap();
        let xi = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xb23800817a8e4504u64,
            0xf7c8d030606cf5d3u64,
            0xc554d5f3a6873b52u64,
            0xde3f28167d9a5291u64,
            0x4f4d918a1865778du64,
            0x132afbf2f8f65a1eu64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xcc9ef3087fed27afu64,
            0x7e81a2f64391f0beu64,
            0xc48938b12beb0fbfu64,
            0x360c79002c1e90f7u64,
            0x751da7c5a9e8babfu64,
            0x0ebd04f9163cec3du64,
        ]))
        .unwrap();
        let yi = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xe3338c82ea2979b4u64,
            0x9a6f91d415db545bu64,
            0xa3ca77e0d9861d1cu64,
            0x28f2f4c58ddda9b9u64,
            0x4619fd312fda5b8au64,
            0x05cedc83f8d1ef6du64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x67b92685e8403d67u64,
            0x60023680e19a4a74u64,
            0x3f08353c8d07f724u64,
            0xd9e2e0af812f9dcfu64,
            0xd14586ab798fd681u64,
            0x0fd1302c1e7f0f46u64,
        ]))
        .unwrap();
        let zi = Fq2 { c0, c1 };
        let mut pi = G2 {
            x: xi,
            y: yi,
            z: zi,
        };
        pi.clear_h();
        let c0 = Fq::from_repr(FqRepr([
            0x4c8957d8d8815b9bu64,
            0x7eeeba08557e6adfu64,
            0x27ec4ebc182fb6eau64,
            0x3813d28668925384u64,
            0x168507538152ff6eu64,
            0x073f71e403e113e7u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xb7166aeff1af65f1u64,
            0x0dfdd3aad2611503u64,
            0x66f71aea8543e538u64,
            0xad827b476a580daeu64,
            0xa01f125180bdfbafu64,
            0x128a5c629c0b95aeu64,
        ]))
        .unwrap();
        let xo = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xc17fa5b0dc489902u64,
            0x0b388a0fc48ad69fu64,
            0x8175bd9a07bfca84u64,
            0x9fbfe48a85acba8du64,
            0x611f3be0a870feb3u64,
            0x04bb1864f86691dcu64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x05bcb86bb3bd9ac5u64,
            0xc12b98541bc9b825u64,
            0xe799456b05496e88u64,
            0xd3e521e467210692u64,
            0xbe800d10cbccee05u64,
            0x0de0e0750127f90fu64,
        ]))
        .unwrap();
        let yo = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0x0eba7a5361d94a4bu64,
            0x6ab1c7c60e71695cu64,
            0xc8bb6f7a7b3a28f0u64,
            0x796502f270c9af00u64,
            0x400ad08f5ce56103u64,
            0x0ebaf76abe831eb9u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x674784654769a83fu64,
            0x3a8ca2cfe26e6c68u64,
            0x7231a53523ca451du64,
            0x3e31339b6cb09cb6u64,
            0xdfec96c2494da8c8u64,
            0x119759a94166869fu64,
        ]))
        .unwrap();
        let zo = Fq2 { c0, c1 };
        let po = G2 {
            x: xo,
            y: yo,
            z: zo,
        };
        assert_eq!(pi, po);
    }
}
