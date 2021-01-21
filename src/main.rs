// static lifetime - variable lasts as long as program is running.  can slow down speed of program.

use std::io::Write;
use std::str;
use std::time::Instant;

// const DIGITS: &'static str = "000102030405060708090A0B0C0D0E0F\
// 		 101112131415161718191A1B1C1D1E1F\
// 		 202122232425262728292A2B2C2D2E2F\
// 		 303132333435363738393A3B3C3D3E3F\
// 		 404142434445464748494A4B4C4D4E4F\
// 		 505152535455565758595A5B5C5D5E5F\
// 		 606162636465666768696A6B6C6D6E6F\
// 		 707172737475767778797A7B7C7D7E7F\
// 		 808182838485868788898A8B8C8D8E8F\
// 		 909192939495969798999A9B9C9D9E9F\
// 		 A0A1A2A3A4A5A6A7A8A9AAABACADAEAF\
// 		 B0B1B2B3B4B5B6B7B8B9BABBBCBDBEBF\
// 		 C0C1C2C3C4C5C6C7C8C9CACBCCCDCECF\
// 		 D0D1D2D3D4D5D6D7D8D9DADBDCDDDEDF\
// 		 E0E1E2E3E4E5E6E7E8E9EAEBECEDEEEF\
//          F0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF";
fn main() {
    // let _s: &'static str = "The String";
    // let _bob = "This is a test";

    // let x = [
    //     0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0,
    //     0x11, 0x21, 0x31, 0x41, 0x51,
    // ];

    let row_size = 16;

    write_byte_buffer(DECLARATION_OF_INDEPENDENCE.as_bytes(), row_size);

    // let a = 0xFF * 2;
    // let b = a + 2;

    // let z = &digits[a..b];

    // let mut now = Instant::now();

    // for i in 0..x.len() {
    //     if i == 0 {
    //         print!("{:04X} : ", i);
    //     } else if i % row_size == 0 {
    //         println!();
    //         print!("{:04X} : ", i);
    //     }

    //     // let a = x[i] * 2;
    //     // let b = a + 2;
    //     // let c = &DIGITS[a..b];
    //     // print!("{} ", c);

    //     print!("{:02X} ", x[i]);
    // }

    // println!();

    // println!("Elapsed : {} usec", now.elapsed().as_micros());

    // now = Instant::now();

    // for i in 0..DECLARATION_OF_INDEPENDENCE.len() {
    //     if i == 0 {
    //         print!("{:04X} : ", i);
    //     } else if i % row_size == 0 {
    //         println!();
    //         print!("{:04X} : ", i);
    //     }
    //     print!("{:02X} ", DECLARATION_OF_INDEPENDENCE.as_bytes()[i]);
    // }

    // println!();

    // println!("Elapsed : {} usec", now.elapsed().as_micros());

 
}

// Writes a string to a UDP socket as if it was a byte buffer
// Assumes MTU of 1500.  Chunks formatted (ASCII) data in intermediate buffer so that data is flushed within MTU size. 
// Displays buffer data in the format (row_size = 16):
// 0000 : xx xx xx xx xx xx xx xx xx xx xx xx xx xx xx xx
// 0010 : xx xx xx xx xx xx xx xx xx xx xx xx xx xx xx xx
// ...
// NNNN : xx xx xx xx xx xx xx xx xx xx xx xx xx xx xx xx
fn write_string(data: &str, row_size: usize) {
    write_byte_buffer(data.as_bytes(), row_size);
}

// Writes a byte buffer to a UDP socket, formatting each byte as ASCII (hex).
// Assumes MTU of 1500.  Chunks formatted (ASCII) data in intermediate buffer so that data is flushed within MTU size. 
// Displays buffer data in the format (row_size = 16):
// 0000 : xx xx xx xx xx xx xx xx xx xx xx xx xx xx xx xx
// 0010 : xx xx xx xx xx xx xx xx xx xx xx xx xx xx xx xx
// ...
// NNNN : xx xx xx xx xx xx xx xx xx xx xx xx xx xx xx xx
fn write_byte_buffer(data: &[u8], row_size:usize) {
    const ROW_0_FORMAT_LENGTH: usize = 7;
    const ROW_N_FORMAT_LENGTH: usize = 8;
    const DATA_FORMAT_LENGTH: usize = 3;
    const BUFFER_SIZE: usize = 1500 * 9 / 10;
    const MAX_WRITE_LENGTH: usize = BUFFER_SIZE - ROW_0_FORMAT_LENGTH - ROW_N_FORMAT_LENGTH - DATA_FORMAT_LENGTH - 1;

    let mut buffer = [0 as u8; BUFFER_SIZE];
    let mut index: usize = 0;
    let mut data_count: usize = 0;
    let mut _check_flush: bool = false;

    for i in 0..data.len() {

        data_count += 1;

        if i == 0 {
            write!(&mut buffer[index..], "{:04X} : ", i).expect("Error writing address at index = 0");
            index += ROW_0_FORMAT_LENGTH;
        } else if i % row_size == 0 {
            write!(&mut buffer[index..], "\n{:04X} : ", i).expect("Error writing adress at index != 0");
            index += ROW_N_FORMAT_LENGTH;
        }

        write!(&mut buffer[index..], "{:02X} ", data[i]).expect("Error writing byte");
        index += DATA_FORMAT_LENGTH;

        if data_count % row_size == 0 {
            data_count = 0;
            if index >= MAX_WRITE_LENGTH {
                // FLUSH BUFFER
                index = 0;
            }
        }
    }

    // FLUSH_BUFFER
}

const DECLARATION_OF_INDEPENDENCE: &'static str =
            "IN CONGRESS, JULY 4, 1776 \
            The unanimous Declaration of the thirteen united States of America \
            When in the Course of human events it becomes necessary for one people to dissolve the political bands which have connected them with another and to assume among the powers of the earth, the separate and equal station to which the Laws of Nature and of Nature's God entitle them, a decent respect to the opinions of mankind requires that they should declare the causes which impel them to the separation. \
            We hold these truths to be self-evident, that all men are created equal, that they are endowed by their Creator with certain unalienable Rights, that among these are Life, Liberty and the pursuit of Happiness. — That to secure these rights, Governments are instituted among Men, deriving their just powers from the consent of the governed, — That whenever any Form of Government becomes destructive of these ends, it is the Right of the People to alter or to abolish it, and to institute new Government, laying its foundation on such principles and organizing its powers in such form, as to them shall seem most likely to effect their Safety and Happiness. Prudence, indeed, will dictate that Governments long established should not be changed for light and transient causes; and accordingly all experience hath shewn that mankind are more disposed to suffer, while evils are sufferable than to right themselves by abolishing the forms to which they are accustomed. But when a long train of abuses and usurpations, pursuing invariably the same Object evinces a design to reduce them under absolute Despotism, it is their right, it is their duty, to throw off such Government, and to provide new Guards for their future security. — Such has been the patient sufferance of these Colonies; and such is now the necessity which constrains them to alter their former Systems of Government. The history of the present King of Great Britain is a history of repeated injuries and usurpations, all having in direct object the establishment of an absolute Tyranny over these States. To prove this, let Facts be submitted to a candid world. \
            He has refused his Assent to Laws, the most wholesome and necessary for the public good. \
            He has forbidden his Governors to pass Laws of immediate and pressing importance, unless suspended in their operation till his Assent should be obtained; and when so suspended, he has utterly neglected to attend to them. \
            He has refused to pass other Laws for the accommodation of large districts of people, unless those people would relinquish the right of Representation in the Legislature, a right inestimable to them and formidable to tyrants only. \
            He has called together legislative bodies at places unusual, uncomfortable, and distant from the depository of their Public Records, for the sole purpose of fatiguing them into compliance with his measures. \
            He has dissolved Representative Houses repeatedly, for opposing with manly firmness his invasions on the rights of the people. \
            He has refused for a long time, after such dissolutions, to cause others to be elected, whereby the Legislative Powers, incapable of Annihilation, have returned to the People at large for their exercise; the State remaining in the mean time exposed to all the dangers of invasion from without, and convulsions within. \
            He has endeavoured to prevent the population of these States; for that purpose obstructing the Laws for Naturalization of Foreigners; refusing to pass others to encourage their migrations hither, and raising the conditions of new Appropriations of Lands. \
            He has obstructed the Administration of Justice by refusing his Assent to Laws for establishing Judiciary Powers. \
            He has made Judges dependent on his Will alone for the tenure of their offices, and the amount and payment of their salaries. \
            He has erected a multitude of New Offices, and sent hither swarms of Officers to harass our people and eat out their substance. \
            He has kept among us, in times of peace, Standing Armies without the Consent of our legislatures. \
            He has affected to render the Military independent of and superior to the Civil Power. \
            He has combined with others to subject us to a jurisdiction foreign to our constitution, and unacknowledged by our laws; giving his Assent to their Acts of pretended Legislation: \
            For quartering large bodies of armed troops among us: \
            For protecting them, by a mock Trial from punishment for any Murders which they should commit on the Inhabitants of these States: \
            For cutting off our Trade with all parts of the world: \
            For imposing Taxes on us without our Consent: \
            For depriving us in many cases, of the benefit of Trial by Jury: \
            For transporting us beyond Seas to be tried for pretended offences: \
            For abolishing the free System of English Laws in a neighbouring Province, establishing therein an Arbitrary government, and enlarging its Boundaries so as to render it at once an example and fit instrument for introducing the same absolute rule into these Colonies \
            For taking away our Charters, abolishing our most valuable Laws and altering fundamentally the Forms of our Governments: \
            For suspending our own Legislatures, and declaring themselves invested with power to legislate for us in all cases whatsoever. \
            He has abdicated Government here, by declaring us out of his Protection and waging War against us. \
            He has plundered our seas, ravaged our coasts, burnt our towns, and destroyed the lives of our people. \
            He is at this time transporting large Armies of foreign Mercenaries to compleat the works of death, desolation, and tyranny, already begun with circumstances of Cruelty & Perfidy scarcely paralleled in the most barbarous ages, and totally unworthy the Head of a civilized nation. \
            He has constrained our fellow Citizens taken Captive on the high Seas to bear Arms against their Country, to become the executioners of their friends and Brethren, or to fall themselves by their Hands. \
            He has excited domestic insurrections amongst us, and has endeavoured to bring on the inhabitants of our frontiers, the merciless Indian Savages whose known rule of warfare, is an undistinguished destruction of all ages, sexes and conditions. \
            In every stage of these Oppressions We have Petitioned for Redress in the most humble terms: Our repeated Petitions have been answered only by repeated injury. A Prince, whose character is thus marked by every act which may define a Tyrant, is unfit to be the ruler of a free people. \
            Nor have We been wanting in attentions to our British brethren. We have warned them from time to time of attempts by their legislature to extend an unwarrantable jurisdiction over us. We have reminded them of the circumstances of our emigration and settlement here. We have appealed to their native justice and magnanimity, and we have conjured them by the ties of our common kindred. to disavow these usurpations, which would inevitably interrupt our connections and correspondence. They too have been deaf to the voice of justice and of consanguinity. We must, therefore, acquiesce in the necessity, which denounces our Separation, and hold them, as we hold the rest of mankind, Enemies in War, in Peace Friends. \
            We, therefore, the Representatives of the United States of America, in General Congress, Assembled, appealing to the Supreme Judge of the world for the rectitude of our intentions, do, in the Name, and by Authority of the good People of these Colonies, solemnly publish and declare, That these United Colonies are, and of Right ought to be Free and Independent States, that they are Absolved from all Allegiance to the British Crown, and that all political connection between them and the State of Great Britain, is and ought to be totally dissolved; and that as Free and Independent States, they have full Power to levy War, conclude Peace contract Alliances, establish Commerce, and to do all other Acts and Things which Independent States may of right do. — And for the support of this Declaration, with a firm reliance on the protection of Divine Providence, we mutually pledge to each other our Lives, our Fortunes and our sacred Honor. \
            — John Hancock \
            New Hampshire: \
            Josiah Bartlett, William Whipple, Matthew Thornton \
            Massachusetts: \
            John Hancock, Samuel Adams, John Adams, Robert Treat Paine, Elbridge Gerry \
            Rhode Island: \
            Stephen Hopkins, William Ellery \
            Connecticut: \
            Roger Sherman, Samuel Huntington, William Williams, Oliver Wolcott \
            New York: \
            William Floyd, Philip Livingston, Francis Lewis, Lewis Morris \
            New Jersey: \
            Richard Stockton, John Witherspoon, Francis Hopkinson, John Hart, Abraham Clark \
            Pennsylvania: \
            Robert Morris, Benjamin Rush, Benjamin Franklin, John Morton, George Clymer, James Smith, George Taylor, James Wilson, George Ross \
            Delaware: \
            Caesar Rodney, George Read, Thomas McKean \
            Maryland: \
            Samuel Chase, William Paca, Thomas Stone, Charles Carroll of Carrollton \
            Virginia: \
            George Wythe, Richard Henry Lee, Thomas Jefferson, Benjamin Harrison, Thomas Nelson, Jr., Francis Lightfoot Lee, Carter Braxton \
            North Carolina: \
            William Hooper, Joseph Hewes, John Penn \
            South Carolina: \
            Edward Rutledge, Thomas Heyward, Jr., Thomas Lynch, Jr., Arthur Middleton \
            Georgia: \
            Button Gwinnett, Lyman Hall, George Walton";
